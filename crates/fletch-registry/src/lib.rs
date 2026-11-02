//! Portable declared-registry contracts, validation and search. No acquisition or ledger verification.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const FLETCH_REGISTRY_SCHEMA: &str = "fletch.registry.v1";
pub const FLETCH_ADAPTER_SOURCES_SCHEMA: &str = "fletch.adapter-sources.v1";
pub const FLETCH_REGISTRY_INDEX_SCHEMA: &str = "fletch.registry-index.v1";
pub const FLETCH_REGISTRY_SEARCH_SCHEMA: &str = "fletch.registry-search.v1";
pub const FLETCH_REGISTRY_VALIDATION_SCHEMA: &str = "fletch.registry-validation.v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceKind {
    Http,
    File,
    Adapter,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSpec {
    pub kind: SourceKind,
    pub url: String,
    #[serde(default)]
    pub headers: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GraphNodeKind {
    Fletch,
    Shaft,
    Quiver,
    Flight,
    LedgerEntry,
    Document,
    Partition,
    Rollup,
    Alias,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GraphEdgeKind {
    Requires,
    ExpandsTo,
    SatisfiedBy,
    Contains,
    DerivedFrom,
    Supersedes,
    Mirrors,
    Cites,
    Documents,
    PointsTo,
    RollsUpTo,
    FoldsOver,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataFormat {
    pub media_type: Option<String>,
    pub encoding: Option<String>,
    pub compression: Option<String>,
    pub container: Option<String>,
    pub schema: Option<String>,
    pub record_shape: Option<String>,
    pub preferred_local: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryEdge {
    pub to: String,
    pub kind: GraphEdgeKind,
    pub label: Option<String>,
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FletchDefinition {
    pub id: String,
    pub node_kind: GraphNodeKind,
    #[serde(default)]
    pub shafts: Vec<SourceSpec>,
    #[serde(default)]
    pub edges: Vec<RegistryEdge>,
    pub format: Option<DataFormat>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FletchRegistry {
    pub schema_version: String,
    pub generated_by: String,
    pub registry_id: String,
    pub fletches: Vec<FletchDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterSourceRecord {
    pub fletch_id: String,
    pub source_kind: SourceKind,
    pub url: String,
    pub header_count: usize,
    pub adapter_owned: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterSourceReport {
    pub schema_version: String,
    pub generated_by: String,
    pub registry_id: String,
    pub fletch_count: usize,
    pub source_count: usize,
    pub adapter_source_count: usize,
    pub sources: Vec<AdapterSourceRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryIndexRow {
    pub registry_id: String,
    pub fletch_id: String,
    pub node_kind: GraphNodeKind,
    pub source_urls: Vec<String>,
    pub source_kinds: Vec<String>,
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryIndexReport {
    pub schema_version: String,
    pub generated_by: String,
    pub registry_count: usize,
    pub fletch_count: usize,
    pub row_count: usize,
    pub rows: Vec<RegistryIndexRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistrySearchReport {
    pub schema_version: String,
    pub generated_by: String,
    pub query: BTreeMap<String, String>,
    pub total_row_count: usize,
    pub matched_row_count: usize,
    pub rows: Vec<RegistryIndexRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryValidationFinding {
    pub fletch_id: Option<String>,
    pub severity: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryValidationReport {
    pub schema_version: String,
    pub generated_by: String,
    pub registry_id: String,
    pub valid: bool,
    pub fletch_count: usize,
    pub source_count: usize,
    pub adapter_source_count: usize,
    pub finding_count: usize,
    pub findings: Vec<RegistryValidationFinding>,
}

pub fn fletch_registry(
    registry_id: impl Into<String>,
    fletches: Vec<FletchDefinition>,
) -> FletchRegistry {
    FletchRegistry {
        schema_version: FLETCH_REGISTRY_SCHEMA.to_string(),
        generated_by: format!("fletch-core/{}", env!("CARGO_PKG_VERSION")),
        registry_id: registry_id.into(),
        fletches,
    }
}

pub fn adapter_sources_from_registry(registry: &FletchRegistry) -> AdapterSourceReport {
    let sources = registry
        .fletches
        .iter()
        .flat_map(|fletch| {
            fletch.shafts.iter().map(|shaft| AdapterSourceRecord {
                fletch_id: fletch.id.clone(),
                source_kind: shaft.kind.clone(),
                url: shaft.url.clone(),
                header_count: shaft.headers.len(),
                adapter_owned: shaft.kind == SourceKind::Adapter,
            })
        })
        .collect::<Vec<_>>();
    AdapterSourceReport {
        schema_version: FLETCH_ADAPTER_SOURCES_SCHEMA.to_string(),
        generated_by: format!("fletch-core/{}", env!("CARGO_PKG_VERSION")),
        registry_id: registry.registry_id.clone(),
        fletch_count: registry.fletches.len(),
        source_count: sources.len(),
        adapter_source_count: sources.iter().filter(|source| source.adapter_owned).count(),
        sources,
    }
}

pub fn registry_index_from_registries(registries: &[FletchRegistry]) -> RegistryIndexReport {
    let rows = registries
        .iter()
        .flat_map(|registry| {
            registry.fletches.iter().map(|definition| {
                let mut source_kinds = definition
                    .shafts
                    .iter()
                    .map(|shaft| source_kind_key(&shaft.kind).to_string())
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>();
                source_kinds.sort();
                RegistryIndexRow {
                    registry_id: registry.registry_id.clone(),
                    fletch_id: definition.id.clone(),
                    node_kind: definition.node_kind.clone(),
                    source_urls: definition
                        .shafts
                        .iter()
                        .map(|shaft| shaft.url.clone())
                        .collect(),
                    source_kinds,
                    tags: definition.tags.clone(),
                    metadata: definition.metadata.clone(),
                }
            })
        })
        .collect::<Vec<_>>();
    let fletch_count = rows
        .iter()
        .map(|row| row.fletch_id.clone())
        .collect::<BTreeSet<_>>()
        .len();
    RegistryIndexReport {
        schema_version: FLETCH_REGISTRY_INDEX_SCHEMA.to_string(),
        generated_by: format!("fletch-core/{}", env!("CARGO_PKG_VERSION")),
        registry_count: registries.len(),
        fletch_count,
        row_count: rows.len(),
        rows,
    }
}

pub fn search_registry_index(
    index: &RegistryIndexReport,
    tags: &[String],
    metadata_filters: &[(String, String)],
    text: Option<&str>,
    offset: usize,
    limit: Option<usize>,
) -> RegistrySearchReport {
    let normalized_text = text.map(|text| text.to_lowercase());
    let rows = index
        .rows
        .iter()
        .filter(|row| {
            tags.iter()
                .all(|tag| row.tags.iter().any(|row_tag| row_tag == tag))
        })
        .filter(|row| {
            metadata_filters
                .iter()
                .all(|(key, value)| row.metadata.get(key) == Some(value))
        })
        .filter(|row| match normalized_text.as_deref() {
            Some(text) => registry_index_row_matches_text(row, text),
            None => true,
        })
        .cloned()
        .collect::<Vec<_>>();
    let sliced_rows = rows
        .iter()
        .skip(offset)
        .take(limit.unwrap_or(usize::MAX))
        .cloned()
        .collect::<Vec<_>>();
    let mut query = BTreeMap::new();
    if !tags.is_empty() {
        query.insert("tags".to_string(), tags.join(","));
    }
    if !metadata_filters.is_empty() {
        query.insert(
            "metadata".to_string(),
            metadata_filters
                .iter()
                .map(|(key, value)| format!("{key}={value}"))
                .collect::<Vec<_>>()
                .join(","),
        );
    }
    if let Some(text) = text {
        query.insert("text".to_string(), text.to_string());
    }
    RegistrySearchReport {
        schema_version: FLETCH_REGISTRY_SEARCH_SCHEMA.to_string(),
        generated_by: format!("fletch-core/{}", env!("CARGO_PKG_VERSION")),
        query,
        total_row_count: index.rows.len(),
        matched_row_count: rows.len(),
        rows: sliced_rows,
    }
}

fn registry_index_row_search_text(row: &RegistryIndexRow) -> String {
    let mut values = vec![
        row.registry_id.clone(),
        row.fletch_id.clone(),
        format!("{:?}", row.node_kind),
    ];
    values.extend(row.source_urls.clone());
    values.extend(row.source_kinds.clone());
    values.extend(row.tags.clone());
    for (key, value) in &row.metadata {
        values.push(key.clone());
        values.push(value.clone());
    }
    values.join(" ").to_lowercase()
}

fn registry_index_row_matches_text(row: &RegistryIndexRow, text: &str) -> bool {
    let haystack = registry_index_row_search_text(row);
    let terms = text.split_whitespace().collect::<Vec<_>>();
    if terms.is_empty() {
        return true;
    }
    haystack.contains(text) || terms.iter().all(|term| haystack.contains(term))
}

pub fn validate_registry(registry: &FletchRegistry) -> RegistryValidationReport {
    let source_report = adapter_sources_from_registry(registry);
    let mut findings = Vec::new();
    if registry.schema_version != FLETCH_REGISTRY_SCHEMA {
        findings.push(RegistryValidationFinding {
            fletch_id: None,
            severity: "error".to_string(),
            code: "invalid-schema".to_string(),
            message: format!(
                "registry schema is {}; expected {}",
                registry.schema_version, FLETCH_REGISTRY_SCHEMA
            ),
        });
    }

    let mut seen = BTreeSet::new();
    for fletch in &registry.fletches {
        if !seen.insert(fletch.id.clone()) {
            findings.push(RegistryValidationFinding {
                fletch_id: Some(fletch.id.clone()),
                severity: "error".to_string(),
                code: "duplicate-fletch-id".to_string(),
                message: "fletch id appears more than once".to_string(),
            });
        }
        if fletch.shafts.is_empty() {
            findings.push(RegistryValidationFinding {
                fletch_id: Some(fletch.id.clone()),
                severity: "error".to_string(),
                code: "missing-shaft".to_string(),
                message: "fletch has no declared shafts".to_string(),
            });
        }
    }

    RegistryValidationReport {
        schema_version: FLETCH_REGISTRY_VALIDATION_SCHEMA.to_string(),
        generated_by: format!("fletch-core/{}", env!("CARGO_PKG_VERSION")),
        registry_id: registry.registry_id.clone(),
        valid: findings.is_empty(),
        fletch_count: registry.fletches.len(),
        source_count: source_report.source_count,
        adapter_source_count: source_report.adapter_source_count,
        finding_count: findings.len(),
        findings,
    }
}

fn source_kind_key(kind: &SourceKind) -> &'static str {
    match kind {
        SourceKind::Http => "http",
        SourceKind::File => "file",
        SourceKind::Adapter => "adapter",
    }
}
