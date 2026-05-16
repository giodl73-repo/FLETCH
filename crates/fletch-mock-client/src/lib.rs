use anyhow::Result;
use fletch_core::{
    cache_manifest, export_quiver, fetch_plan_with_kind, fetch_to_cache, fletch_registry,
    graph_from_manifest_with_node_kinds, graph_from_registry, import_quiver,
    inspect_cache_manifest, plan_cache_prune, CacheFreshnessStatus, CacheManifest,
    CacheObjectStatus, DataFormat, FetchOptions, FletchDefinition, FletchGraph, FletchRegistry,
    FreshnessPolicy, GraphEdgeKind, GraphNodeKind, GraphNodeKindHints, PrunePlan, RegistryEdge,
    SourceKind, SourceSpec,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MockClientReport {
    pub client: String,
    pub registry_path: String,
    pub manifest_path: String,
    pub fetched_fletches: Vec<String>,
    pub verified_count: usize,
    pub fresh_count: usize,
    pub prune_count: usize,
    pub quiver_path: String,
    pub staged_quiver_root: String,
    pub staged_import_count: usize,
    pub graph_path: String,
    pub graph_node_count: usize,
    pub graph_edge_count: usize,
    pub threat_query: ThreatQueryReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ThreatQueryReport {
    pub by_year: Vec<ThreatYearSummary>,
    pub by_city: Vec<ThreatCitySummary>,
    pub by_villain: Vec<ThreatVillainSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ThreatYearSummary {
    pub year: String,
    pub partitions: u64,
    pub threat_count: u64,
    pub omega_events: u64,
    pub cities_impacted: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ThreatCitySummary {
    pub city: String,
    pub threat_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ThreatVillainSummary {
    pub villain: String,
    pub appearances: u64,
}

pub fn run_mock_client(workspace_root: impl AsRef<Path>) -> Result<MockClientReport> {
    let workspace_root = workspace_root.as_ref();
    let source_root = workspace_root.join("source");
    let cache_root = workspace_root.join("cache");
    let registry_path = workspace_root.join("mock-registry.json");
    let manifest_path = workspace_root.join("mock-manifest.json");

    std::fs::create_dir_all(&source_root)?;
    std::fs::write(
        source_root.join("villain-index.json"),
        br#"{"archive":"justice-league","watchlist":["darkseid","lex-luthor"]}"#,
    )?;
    std::fs::write(
        source_root.join("darkseid-casefile.json"),
        br#"{"case":"darkseid","sector":"apokolips","threat_level":"omega"}"#,
    )?;
    std::fs::write(
        source_root.join("threats-2025-05-15.json"),
        br#"{"partition":"date","date":"2025-05-15","rollups":["justice-league:threats:year:2025"],"measures":{"threat_count":1,"omega_events":0,"cities_impacted":1},"threats":[{"villain":"lex-luthor","city":"metropolis"}]}"#,
    )?;
    std::fs::write(
        source_root.join("threats-2026-05-15.json"),
        br#"{"partition":"date","date":"2026-05-15","rollups":["justice-league:threats:year:2026"],"measures":{"threat_count":2,"omega_events":1,"cities_impacted":2},"threats":[{"villain":"lex-luthor","city":"metropolis"},{"villain":"grodd","city":"central-city"}]}"#,
    )?;

    let registry = villain_files_registry(&source_root);
    std::fs::write(&registry_path, serde_json::to_string_pretty(&registry)?)?;

    let mut fetched_fletches = Vec::new();
    let mut entries = Vec::new();
    for definition in &registry.fletches {
        let Some(shaft) = definition.shafts.first() else {
            continue;
        };
        let plan =
            fetch_plan_with_kind(definition.id.clone(), shaft.url.clone(), shaft.kind.clone())?;
        let outcome = fetch_to_cache(&plan, FetchOptions::new(&cache_root))?;
        fetched_fletches.push(definition.id.clone());
        entries.push(outcome.entry);
    }

    let manifest = cache_manifest(cache_root.display().to_string(), entries)?;
    std::fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)?;

    let orphan_path = cache_root
        .join("objects")
        .join("sha256")
        .join("trick-arrow-orphan");
    if let Some(parent) = orphan_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&orphan_path, b"orphaned trick-arrow object")?;

    let statuses = inspect_cache_manifest(&manifest, &FreshnessPolicy::Immutable)?;
    let prune = plan_cache_prune(&manifest)?;
    let threat_query = query_threat_partitions(&manifest)?;
    let exported = export_quiver(
        &manifest,
        "justice-league:villain-files:demo",
        workspace_root.join("quivers").join("villain-files"),
    )?;
    let imported = import_quiver(
        exported.path.parent().unwrap(),
        workspace_root.join("offline-cache"),
    )?;
    let staged_statuses =
        inspect_cache_manifest(&imported.staged_manifest, &FreshnessPolicy::Immutable)?;
    let graph = villain_files_graph(&manifest, &registry);
    let graph_path = workspace_root.join("mock-graph.json");
    std::fs::write(&graph_path, serde_json::to_string_pretty(&graph)?)?;

    Ok(report(
        manifest_path,
        registry_path,
        fetched_fletches,
        &statuses,
        &prune,
        exported.path,
        imported.stage_root,
        staged_statuses.len(),
        graph_path,
        graph.nodes.len(),
        graph.edges.len(),
        threat_query,
    ))
}

fn report(
    manifest_path: PathBuf,
    registry_path: PathBuf,
    fetched_fletches: Vec<String>,
    statuses: &[fletch_core::CacheStatus],
    prune: &PrunePlan,
    quiver_path: PathBuf,
    staged_quiver_root: PathBuf,
    staged_import_count: usize,
    graph_path: PathBuf,
    graph_node_count: usize,
    graph_edge_count: usize,
    threat_query: ThreatQueryReport,
) -> MockClientReport {
    MockClientReport {
        client: "justice-league-villain-files-mock".to_string(),
        registry_path: registry_path.display().to_string(),
        manifest_path: manifest_path.display().to_string(),
        fetched_fletches,
        verified_count: statuses
            .iter()
            .filter(|status| status.object_status == CacheObjectStatus::Verified)
            .count(),
        fresh_count: statuses
            .iter()
            .filter(|status| status.freshness_status == CacheFreshnessStatus::Fresh)
            .count(),
        prune_count: prune.prune_count,
        quiver_path: quiver_path.display().to_string(),
        staged_quiver_root: staged_quiver_root.display().to_string(),
        staged_import_count,
        graph_path: graph_path.display().to_string(),
        graph_node_count,
        graph_edge_count,
        threat_query,
    }
}

fn villain_files_graph(manifest: &CacheManifest, registry: &FletchRegistry) -> FletchGraph {
    let registry_graph = graph_from_registry(registry);
    graph_from_manifest_with_node_kinds(
        manifest,
        &registry_node_kind_hints(registry),
        registry_graph.nodes,
        registry_graph.edges,
    )
}

fn registry_node_kind_hints(registry: &FletchRegistry) -> GraphNodeKindHints {
    registry
        .fletches
        .iter()
        .map(|definition| (definition.id.clone(), definition.node_kind.clone()))
        .collect()
}

fn villain_files_registry(source_root: &Path) -> FletchRegistry {
    fletch_registry(
        "justice-league:villain-files",
        vec![
            fletch_definition(
                "justice-league:villains:index",
                GraphNodeKind::Fletch,
                Some(source_root.join("villain-index.json")),
                vec![
                    edge(
                        "justice-league:villain-file:darkseid",
                        GraphEdgeKind::ExpandsTo,
                    ),
                    edge(
                        "justice-league:threats:date:2025-05-15",
                        GraphEdgeKind::ExpandsTo,
                    ),
                    edge(
                        "justice-league:threats:date:2026-05-15",
                        GraphEdgeKind::ExpandsTo,
                    ),
                ],
                Some("justice-league.villain-index.v1"),
            ),
            fletch_definition(
                "justice-league:villain-file:darkseid",
                GraphNodeKind::Fletch,
                Some(source_root.join("darkseid-casefile.json")),
                Vec::new(),
                Some("justice-league.casefile.v1"),
            ),
            fletch_definition(
                "justice-league:threats:date:2025-05-15",
                GraphNodeKind::Partition,
                Some(source_root.join("threats-2025-05-15.json")),
                vec![edge(
                    "justice-league:threats:year:2025",
                    GraphEdgeKind::RollsUpTo,
                )],
                Some("justice-league.threat-partition.v1"),
            ),
            fletch_definition(
                "justice-league:threats:date:2026-05-15",
                GraphNodeKind::Partition,
                Some(source_root.join("threats-2026-05-15.json")),
                vec![edge(
                    "justice-league:threats:year:2026",
                    GraphEdgeKind::RollsUpTo,
                )],
                Some("justice-league.threat-partition.v1"),
            ),
            fletch_definition(
                "justice-league:threats:year:2025",
                GraphNodeKind::Rollup,
                None,
                Vec::new(),
                None,
            ),
            fletch_definition(
                "justice-league:threats:year:2026",
                GraphNodeKind::Rollup,
                None,
                Vec::new(),
                None,
            ),
        ],
    )
}

fn fletch_definition(
    id: &str,
    node_kind: GraphNodeKind,
    source_path: Option<PathBuf>,
    edges: Vec<RegistryEdge>,
    schema: Option<&str>,
) -> FletchDefinition {
    FletchDefinition {
        id: id.to_string(),
        node_kind,
        shafts: source_path
            .map(|path| {
                vec![SourceSpec {
                    kind: SourceKind::File,
                    url: path.display().to_string(),
                    headers: BTreeMap::new(),
                }]
            })
            .unwrap_or_default(),
        edges,
        format: schema.map(|schema| DataFormat {
            media_type: Some("application/json".to_string()),
            encoding: Some("utf-8".to_string()),
            compression: None,
            container: None,
            schema: Some(schema.to_string()),
            record_shape: Some("json-object".to_string()),
            preferred_local: None,
        }),
        tags: vec!["mock".to_string()],
        metadata: BTreeMap::new(),
    }
}

fn edge(to: &str, kind: GraphEdgeKind) -> RegistryEdge {
    RegistryEdge {
        to: to.to_string(),
        kind,
        label: None,
        metadata: BTreeMap::new(),
    }
}

#[derive(Debug, Deserialize)]
struct ThreatPartition {
    date: String,
    measures: ThreatMeasures,
    threats: Vec<ThreatEvent>,
}

#[derive(Debug, Deserialize)]
struct ThreatMeasures {
    threat_count: u64,
    omega_events: u64,
    cities_impacted: u64,
}

#[derive(Debug, Deserialize)]
struct ThreatEvent {
    villain: String,
    city: String,
}

#[derive(Debug, Default)]
struct ThreatYearAccumulator {
    partitions: u64,
    threat_count: u64,
    omega_events: u64,
    cities_impacted: u64,
}

fn query_threat_partitions(manifest: &CacheManifest) -> Result<ThreatQueryReport> {
    let mut by_year = BTreeMap::<String, ThreatYearAccumulator>::new();
    let mut by_city = BTreeMap::<String, u64>::new();
    let mut by_villain = BTreeMap::<String, u64>::new();

    for entry in manifest
        .entries
        .iter()
        .filter(|entry| entry.dataset_id.starts_with("justice-league:threats:date:"))
    {
        let path = cache_object_path(&manifest.cache_root, &entry.relative_path);
        let partition: ThreatPartition = serde_json::from_slice(&std::fs::read(path)?)?;
        let year = partition.date.chars().take(4).collect::<String>();
        let year_entry = by_year.entry(year).or_default();
        year_entry.partitions += 1;
        year_entry.threat_count += partition.measures.threat_count;
        year_entry.omega_events += partition.measures.omega_events;
        year_entry.cities_impacted += partition.measures.cities_impacted;

        for threat in partition.threats {
            *by_city.entry(threat.city).or_default() += 1;
            *by_villain.entry(threat.villain).or_default() += 1;
        }
    }

    Ok(ThreatQueryReport {
        by_year: by_year
            .into_iter()
            .map(|(year, summary)| ThreatYearSummary {
                year,
                partitions: summary.partitions,
                threat_count: summary.threat_count,
                omega_events: summary.omega_events,
                cities_impacted: summary.cities_impacted,
            })
            .collect(),
        by_city: by_city
            .into_iter()
            .map(|(city, threat_count)| ThreatCitySummary { city, threat_count })
            .collect(),
        by_villain: by_villain
            .into_iter()
            .map(|(villain, appearances)| ThreatVillainSummary {
                villain,
                appearances,
            })
            .collect(),
    })
}

fn cache_object_path(cache_root: &str, relative_path: &str) -> PathBuf {
    relative_path
        .split('/')
        .fold(PathBuf::from(cache_root), |path, part| path.join(part))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn mock_client_exercises_fetch_status_and_prune() {
        let root = unique_temp_dir("mock-client");

        let report = run_mock_client(&root).unwrap();

        assert_eq!(report.fetched_fletches.len(), 4);
        assert_eq!(report.verified_count, 4);
        assert_eq!(report.fresh_count, 4);
        assert_eq!(report.prune_count, 1);
        assert_eq!(report.staged_import_count, 4);
        assert!(Path::new(&report.registry_path).exists());
        assert!(Path::new(&report.quiver_path).exists());
        assert!(Path::new(&report.staged_quiver_root).exists());
        assert!(Path::new(&report.graph_path).exists());
        assert_eq!(report.graph_node_count, 18);
        assert_eq!(report.graph_edge_count, 17);
        assert_eq!(
            report
                .threat_query
                .by_year
                .iter()
                .map(|summary| (&summary.year, summary.threat_count))
                .collect::<Vec<_>>(),
            vec![(&"2025".to_string(), 1), (&"2026".to_string(), 2)]
        );
        assert_eq!(
            report
                .threat_query
                .by_city
                .iter()
                .map(|summary| (&summary.city, summary.threat_count))
                .collect::<Vec<_>>(),
            vec![
                (&"central-city".to_string(), 1),
                (&"metropolis".to_string(), 2)
            ]
        );
        assert_eq!(
            report
                .threat_query
                .by_villain
                .iter()
                .map(|summary| (&summary.villain, summary.appearances))
                .collect::<Vec<_>>(),
            vec![(&"grodd".to_string(), 1), (&"lex-luthor".to_string(), 2)]
        );
        assert_eq!(
            report.fetched_fletches,
            vec![
                "justice-league:villains:index",
                "justice-league:villain-file:darkseid",
                "justice-league:threats:date:2025-05-15",
                "justice-league:threats:date:2026-05-15"
            ]
        );
        assert!(Path::new(&report.manifest_path).exists());

        let _ = std::fs::remove_dir_all(root);
    }

    fn unique_temp_dir(label: &str) -> PathBuf {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let dir =
            std::env::temp_dir().join(format!("fletch-{label}-{}-{millis}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }
}
