use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use thiserror::Error;

pub const FLETCH_PLAN_SCHEMA: &str = "fletch.plan.v1";
pub const FLETCH_MANIFEST_SCHEMA: &str = "fletch.cache-manifest.v1";
pub const FLETCH_QUIVER_SCHEMA: &str = "fletch.quiver.v1";
pub const FLETCH_GRAPH_SCHEMA: &str = "fletch.graph.v1";

#[derive(Debug, Error)]
pub enum FletchError {
    #[error("[PLAN] dataset id must not be empty")]
    EmptyDatasetId,
    #[error("[PLAN] source URL must not be empty")]
    EmptySourceUrl,
    #[error("[QUIVER] quiver id must not be empty")]
    EmptyQuiverId,
    #[error("[CACHE] cache entry {dataset_id} has invalid sha256: {sha256}")]
    InvalidSha256 { dataset_id: String, sha256: String },
    #[error("[FETCH] source kind {kind:?} cannot be fetched by generic execution")]
    UnsupportedSourceKind { kind: SourceKind },
    #[error("[FETCH] failed to read {path}: {source}")]
    ReadSource {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("[FETCH] failed to write {path}: {source}")]
    WriteCache {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("[FETCH] HTTP fetch failed for {url}: {source}")]
    HttpFetch {
        url: String,
        #[source]
        source: reqwest::Error,
    },
    #[error("[FETCH] invalid HTTP header name {name}: {source}")]
    InvalidHeaderName {
        name: String,
        #[source]
        source: reqwest::header::InvalidHeaderName,
    },
    #[error("[FETCH] invalid HTTP header value for {name}: {source}")]
    InvalidHeaderValue {
        name: String,
        #[source]
        source: reqwest::header::InvalidHeaderValue,
    },
    #[error("[FETCH] bandwidth limit must be greater than zero bytes per second")]
    InvalidBandwidthLimit,
    #[error("[OFFLINE] cache entry {dataset_id} is missing and live fetches are disabled")]
    OfflineCacheMiss { dataset_id: String },
    #[error("[CACHE] relative cache path is unsafe: {relative_path}")]
    UnsafeCachePath { relative_path: String },
    #[error(
        "[CACHE] cache entry {dataset_id} is not verified for quiver export/import: {status:?}"
    )]
    CacheObjectUnverified {
        dataset_id: String,
        status: CacheObjectStatus,
    },
    #[error("[QUIVER] failed to read quiver JSON {path}: {source}")]
    ReadQuiverJson {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("[QUIVER] failed to write quiver JSON {path}: {source}")]
    WriteQuiverJson {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("[VERIFY] checksum mismatch for {dataset_id}: expected {expected}, got {actual}")]
    ChecksumMismatch {
        dataset_id: String,
        expected: String,
        actual: String,
    },
    #[error("[TIME] system clock is before Unix epoch")]
    ClockBeforeEpoch,
}

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
pub enum FreshnessPolicy {
    Immutable,
    MaxAgeDays(u32),
    AlwaysCheck,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CachePolicy {
    pub freshness: FreshnessPolicy,
    pub allow_offline: bool,
    pub resumable: bool,
}

impl Default for CachePolicy {
    fn default() -> Self {
        Self {
            freshness: FreshnessPolicy::Immutable,
            allow_offline: true,
            resumable: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FetchPlan {
    pub schema_version: String,
    pub dataset_id: String,
    pub version: Option<String>,
    pub source: SourceSpec,
    pub cache_policy: CachePolicy,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheEntry {
    pub dataset_id: String,
    pub version: Option<String>,
    pub source_url: String,
    pub cache_key: String,
    pub relative_path: String,
    pub sha256: String,
    pub bytes: u64,
    pub fetched_at_ms: u64,
    pub verified: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheManifest {
    pub schema_version: String,
    pub generated_by: String,
    pub cache_root: String,
    pub entries: Vec<CacheEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CacheObjectStatus {
    Verified,
    Missing,
    HashMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CacheFreshnessStatus {
    Fresh,
    Stale,
    Missing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheStatus {
    pub dataset_id: String,
    pub cache_key: String,
    pub relative_path: String,
    pub absolute_path: String,
    pub expected_sha256: String,
    pub actual_sha256: Option<String>,
    pub expected_bytes: u64,
    pub actual_bytes: Option<u64>,
    pub object_status: CacheObjectStatus,
    pub freshness_status: CacheFreshnessStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PruneCandidate {
    pub relative_path: String,
    pub absolute_path: String,
    pub bytes: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrunePlan {
    pub cache_root: String,
    pub keep_count: usize,
    pub prune_count: usize,
    pub prune_bytes: u64,
    pub candidates: Vec<PruneCandidate>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuiverManifest {
    pub schema_version: String,
    pub generated_by: String,
    pub quiver_id: String,
    pub entries: Vec<CacheEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuiverExport {
    pub manifest: QuiverManifest,
    pub path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuiverImport {
    pub quiver_manifest: QuiverManifest,
    pub staged_manifest: CacheManifest,
    pub stage_root: PathBuf,
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
pub struct GraphNode {
    pub id: String,
    pub kind: GraphNodeKind,
    pub label: String,
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
    pub kind: GraphEdgeKind,
    pub label: Option<String>,
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FletchGraph {
    pub schema_version: String,
    pub generated_by: String,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

pub fn fetch_plan(
    dataset_id: impl Into<String>,
    source_url: impl Into<String>,
) -> Result<FetchPlan, FletchError> {
    fetch_plan_with_kind(dataset_id, source_url, SourceKind::Http)
}

pub fn fetch_plan_with_kind(
    dataset_id: impl Into<String>,
    source_url: impl Into<String>,
    source_kind: SourceKind,
) -> Result<FetchPlan, FletchError> {
    let dataset_id = dataset_id.into();
    let source_url = source_url.into();
    if dataset_id.trim().is_empty() {
        return Err(FletchError::EmptyDatasetId);
    }
    if source_url.trim().is_empty() {
        return Err(FletchError::EmptySourceUrl);
    }
    Ok(FetchPlan {
        schema_version: FLETCH_PLAN_SCHEMA.to_string(),
        dataset_id,
        version: None,
        source: SourceSpec {
            kind: source_kind,
            url: source_url,
            headers: BTreeMap::new(),
        },
        cache_policy: CachePolicy::default(),
        tags: Vec::new(),
        metadata: BTreeMap::new(),
    })
}

pub fn cache_key(plan: &FetchPlan) -> String {
    let mut hasher = Sha256::new();
    hasher.update(plan.dataset_id.as_bytes());
    hasher.update([0]);
    if let Some(version) = &plan.version {
        hasher.update(version.as_bytes());
    }
    hasher.update([0]);
    hasher.update(source_kind_key(&plan.source.kind).as_bytes());
    hasher.update([0]);
    hasher.update(plan.source.url.as_bytes());
    format!("sha256:{:x}", hasher.finalize())
}

pub fn cache_manifest(
    cache_root: impl Into<String>,
    entries: Vec<CacheEntry>,
) -> Result<CacheManifest, FletchError> {
    for entry in &entries {
        if !entry.sha256.starts_with("sha256:") || entry.sha256.len() != 71 {
            return Err(FletchError::InvalidSha256 {
                dataset_id: entry.dataset_id.clone(),
                sha256: entry.sha256.clone(),
            });
        }
    }
    Ok(CacheManifest {
        schema_version: FLETCH_MANIFEST_SCHEMA.to_string(),
        generated_by: format!("fletch-core/{}", env!("CARGO_PKG_VERSION")),
        cache_root: cache_root.into(),
        entries,
    })
}

pub fn cache_list(manifest: &CacheManifest) -> &[CacheEntry] {
    &manifest.entries
}

pub fn inspect_cache_manifest(
    manifest: &CacheManifest,
    freshness: &FreshnessPolicy,
) -> Result<Vec<CacheStatus>, FletchError> {
    manifest
        .entries
        .iter()
        .map(|entry| inspect_cache_entry(&manifest.cache_root, entry, freshness))
        .collect()
}

pub fn plan_cache_prune(manifest: &CacheManifest) -> Result<PrunePlan, FletchError> {
    let cache_root = PathBuf::from(&manifest.cache_root);
    let mut referenced = BTreeSet::new();
    for entry in &manifest.entries {
        referenced.insert(normalize_relative_cache_path(&entry.relative_path)?);
    }

    let object_root = cache_root.join("objects");
    let mut candidates = Vec::new();
    if object_root.exists() {
        collect_prune_candidates(&cache_root, &object_root, &referenced, &mut candidates)?;
    }
    candidates.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    let prune_bytes = candidates.iter().map(|candidate| candidate.bytes).sum();

    Ok(PrunePlan {
        cache_root: manifest.cache_root.clone(),
        keep_count: referenced.len(),
        prune_count: candidates.len(),
        prune_bytes,
        candidates,
    })
}

pub fn export_quiver(
    manifest: &CacheManifest,
    quiver_id: impl Into<String>,
    quiver_root: impl AsRef<Path>,
) -> Result<QuiverExport, FletchError> {
    let quiver_id = quiver_id.into();
    if quiver_id.trim().is_empty() {
        return Err(FletchError::EmptyQuiverId);
    }
    let statuses = inspect_cache_manifest(manifest, &FreshnessPolicy::Immutable)?;
    for status in statuses {
        if status.object_status != CacheObjectStatus::Verified {
            return Err(FletchError::CacheObjectUnverified {
                dataset_id: status.dataset_id,
                status: status.object_status,
            });
        }
    }

    let quiver_root = quiver_root.as_ref();
    std::fs::create_dir_all(quiver_root).map_err(|source| FletchError::WriteCache {
        path: quiver_root.display().to_string(),
        source,
    })?;
    for entry in &manifest.entries {
        let relative_path = normalize_relative_cache_path(&entry.relative_path)?;
        let source = cache_path(Path::new(&manifest.cache_root), &relative_path);
        let destination = cache_path(quiver_root, &relative_path);
        copy_file(&source, &destination)?;
    }

    let quiver_manifest = QuiverManifest {
        schema_version: FLETCH_QUIVER_SCHEMA.to_string(),
        generated_by: format!("fletch-core/{}", env!("CARGO_PKG_VERSION")),
        quiver_id,
        entries: manifest.entries.clone(),
    };
    let manifest_path = quiver_root.join("quiver.json");
    write_quiver_manifest(&manifest_path, &quiver_manifest)?;

    Ok(QuiverExport {
        manifest: quiver_manifest,
        path: manifest_path,
    })
}

pub fn import_quiver(
    quiver_root: impl AsRef<Path>,
    cache_root: impl AsRef<Path>,
) -> Result<QuiverImport, FletchError> {
    let quiver_root = quiver_root.as_ref();
    let quiver_manifest = read_quiver_manifest(&quiver_root.join("quiver.json"))?;
    let stage_root = cache_root
        .as_ref()
        .join("staged")
        .join("quivers")
        .join(safe_path_label(&quiver_manifest.quiver_id));
    std::fs::create_dir_all(&stage_root).map_err(|source| FletchError::WriteCache {
        path: stage_root.display().to_string(),
        source,
    })?;

    for entry in &quiver_manifest.entries {
        let relative_path = normalize_relative_cache_path(&entry.relative_path)?;
        let source = cache_path(quiver_root, &relative_path);
        let destination = cache_path(&stage_root, &relative_path);
        copy_file(&source, &destination)?;
    }

    let staged_manifest = cache_manifest(
        stage_root.display().to_string(),
        quiver_manifest.entries.clone(),
    )?;
    let statuses = inspect_cache_manifest(&staged_manifest, &FreshnessPolicy::Immutable)?;
    for status in statuses {
        if status.object_status != CacheObjectStatus::Verified {
            return Err(FletchError::CacheObjectUnverified {
                dataset_id: status.dataset_id,
                status: status.object_status,
            });
        }
    }

    Ok(QuiverImport {
        quiver_manifest,
        staged_manifest,
        stage_root,
    })
}

pub fn graph_from_manifest(manifest: &CacheManifest) -> FletchGraph {
    graph_from_manifest_with_extra(manifest, Vec::new(), Vec::new())
}

pub fn graph_from_manifest_with_extra(
    manifest: &CacheManifest,
    extra_nodes: Vec<GraphNode>,
    extra_edges: Vec<GraphEdge>,
) -> FletchGraph {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut seen_nodes = BTreeSet::new();

    for entry in &manifest.entries {
        let fletch_id = graph_fletch_id(&entry.dataset_id);
        let shaft_id = graph_shaft_id(&entry.cache_key);
        let ledger_id = graph_ledger_id(&entry.cache_key);

        let mut fletch_metadata = BTreeMap::new();
        fletch_metadata.insert("cache_key".to_string(), entry.cache_key.clone());
        if let Some(version) = &entry.version {
            fletch_metadata.insert("version".to_string(), version.clone());
        }
        push_graph_node(
            &mut nodes,
            &mut seen_nodes,
            GraphNode {
                id: fletch_id.clone(),
                kind: GraphNodeKind::Fletch,
                label: entry.dataset_id.clone(),
                metadata: fletch_metadata,
            },
        );

        let mut shaft_metadata = BTreeMap::new();
        shaft_metadata.insert("source_url".to_string(), entry.source_url.clone());
        push_graph_node(
            &mut nodes,
            &mut seen_nodes,
            GraphNode {
                id: shaft_id.clone(),
                kind: GraphNodeKind::Shaft,
                label: entry.source_url.clone(),
                metadata: shaft_metadata,
            },
        );

        let mut ledger_metadata = BTreeMap::new();
        ledger_metadata.insert("relative_path".to_string(), entry.relative_path.clone());
        ledger_metadata.insert("sha256".to_string(), entry.sha256.clone());
        ledger_metadata.insert("bytes".to_string(), entry.bytes.to_string());
        ledger_metadata.insert("verified".to_string(), entry.verified.to_string());
        ledger_metadata.insert("cache_root".to_string(), manifest.cache_root.clone());
        push_graph_node(
            &mut nodes,
            &mut seen_nodes,
            GraphNode {
                id: ledger_id.clone(),
                kind: GraphNodeKind::LedgerEntry,
                label: entry.relative_path.clone(),
                metadata: ledger_metadata,
            },
        );

        edges.push(GraphEdge {
            from: fletch_id.clone(),
            to: shaft_id,
            kind: GraphEdgeKind::SatisfiedBy,
            label: None,
            metadata: BTreeMap::new(),
        });
        edges.push(GraphEdge {
            from: ledger_id,
            to: fletch_id,
            kind: GraphEdgeKind::Documents,
            label: None,
            metadata: BTreeMap::new(),
        });
    }

    for node in extra_nodes {
        push_graph_node(&mut nodes, &mut seen_nodes, node);
    }
    edges.extend(extra_edges);

    FletchGraph {
        schema_version: FLETCH_GRAPH_SCHEMA.to_string(),
        generated_by: format!("fletch-core/{}", env!("CARGO_PKG_VERSION")),
        nodes,
        edges,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FetchOptions {
    pub cache_root: PathBuf,
    pub expected_sha256: Option<String>,
    pub fetched_at_ms: Option<u64>,
    pub max_bytes_per_second: Option<u64>,
    pub force: bool,
    pub offline: bool,
}

impl FetchOptions {
    pub fn new(cache_root: impl Into<PathBuf>) -> Self {
        Self {
            cache_root: cache_root.into(),
            expected_sha256: None,
            fetched_at_ms: None,
            max_bytes_per_second: None,
            force: false,
            offline: false,
        }
    }

    pub fn with_expected_sha256(mut self, expected_sha256: impl Into<String>) -> Self {
        self.expected_sha256 = Some(expected_sha256.into());
        self
    }

    pub fn with_fetched_at_ms(mut self, fetched_at_ms: u64) -> Self {
        self.fetched_at_ms = Some(fetched_at_ms);
        self
    }

    pub fn with_max_bytes_per_second(mut self, max_bytes_per_second: u64) -> Self {
        self.max_bytes_per_second = Some(max_bytes_per_second);
        self
    }

    pub fn with_force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }

    pub fn with_offline(mut self, offline: bool) -> Self {
        self.offline = offline;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FetchOutcome {
    pub entry: CacheEntry,
    pub path: PathBuf,
}

pub fn fetch_to_cache(
    plan: &FetchPlan,
    options: FetchOptions,
) -> Result<FetchOutcome, FletchError> {
    if options.max_bytes_per_second == Some(0) {
        return Err(FletchError::InvalidBandwidthLimit);
    }

    let key = cache_key(plan);
    let relative_path = relative_cache_path(&key);
    let destination = cache_path(&options.cache_root, &relative_path);
    let temp_path = temp_path_for(&destination);

    if !options.force && cache_hit_is_fresh(&destination, &plan.cache_policy.freshness)? {
        return cache_hit_outcome(plan, &key, relative_path, destination, &options);
    }

    if options.offline {
        return Err(FletchError::OfflineCacheMiss {
            dataset_id: plan.dataset_id.clone(),
        });
    }

    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent).map_err(|source| FletchError::WriteCache {
            path: parent.display().to_string(),
            source,
        })?;
    }

    let (sha256, bytes) = match plan.source.kind {
        SourceKind::Http => {
            let client = reqwest::blocking::Client::new();
            let mut request = client.get(&plan.source.url);
            for (name, value) in &plan.source.headers {
                let header_name = reqwest::header::HeaderName::from_bytes(name.as_bytes())
                    .map_err(|source| FletchError::InvalidHeaderName {
                        name: name.clone(),
                        source,
                    })?;
                let header_value =
                    reqwest::header::HeaderValue::from_str(value).map_err(|source| {
                        FletchError::InvalidHeaderValue {
                            name: name.clone(),
                            source,
                        }
                    })?;
                request = request.header(header_name, header_value);
            }

            let response = request.send().map_err(|source| FletchError::HttpFetch {
                url: plan.source.url.clone(),
                source,
            })?;
            let mut response =
                response
                    .error_for_status()
                    .map_err(|source| FletchError::HttpFetch {
                        url: plan.source.url.clone(),
                        source,
                    })?;
            write_stream_to_temp(&mut response, &temp_path, options.max_bytes_per_second)?
        }
        SourceKind::File => {
            let path = file_source_path(&plan.source.url);
            let mut file = File::open(&path).map_err(|source| FletchError::ReadSource {
                path: path.display().to_string(),
                source,
            })?;
            write_stream_to_temp(&mut file, &temp_path, options.max_bytes_per_second)?
        }
        SourceKind::Adapter => {
            return Err(FletchError::UnsupportedSourceKind {
                kind: plan.source.kind.clone(),
            });
        }
    };

    if let Some(expected) = &options.expected_sha256 {
        if &sha256 != expected {
            let _ = std::fs::remove_file(&temp_path);
            return Err(FletchError::ChecksumMismatch {
                dataset_id: plan.dataset_id.clone(),
                expected: expected.clone(),
                actual: sha256,
            });
        }
    }

    promote_temp(&temp_path, &destination)?;

    let fetched_at_ms = match options.fetched_at_ms {
        Some(value) => value,
        None => now_ms()?,
    };
    let entry = CacheEntry {
        dataset_id: plan.dataset_id.clone(),
        version: plan.version.clone(),
        source_url: plan.source.url.clone(),
        cache_key: key,
        relative_path,
        sha256,
        bytes,
        fetched_at_ms,
        verified: true,
    };

    Ok(FetchOutcome {
        entry,
        path: destination,
    })
}

fn cache_hit_is_fresh(
    destination: &Path,
    freshness: &FreshnessPolicy,
) -> Result<bool, FletchError> {
    if !destination.exists() {
        return Ok(false);
    }
    match freshness {
        FreshnessPolicy::Immutable => Ok(true),
        FreshnessPolicy::AlwaysCheck => Ok(false),
        FreshnessPolicy::MaxAgeDays(days) => {
            let metadata = destination
                .metadata()
                .map_err(|source| FletchError::ReadSource {
                    path: destination.display().to_string(),
                    source,
                })?;
            let modified = metadata
                .modified()
                .map_err(|source| FletchError::ReadSource {
                    path: destination.display().to_string(),
                    source,
                })?;
            let age = SystemTime::now()
                .duration_since(modified)
                .unwrap_or_else(|_| Duration::ZERO);
            Ok(age <= Duration::from_secs(*days as u64 * 24 * 60 * 60))
        }
    }
}

fn inspect_cache_entry(
    cache_root: &str,
    entry: &CacheEntry,
    freshness: &FreshnessPolicy,
) -> Result<CacheStatus, FletchError> {
    let relative_path = normalize_relative_cache_path(&entry.relative_path)?;
    let path = cache_path(Path::new(cache_root), &relative_path);
    if !path.exists() {
        return Ok(CacheStatus {
            dataset_id: entry.dataset_id.clone(),
            cache_key: entry.cache_key.clone(),
            relative_path,
            absolute_path: path.display().to_string(),
            expected_sha256: entry.sha256.clone(),
            actual_sha256: None,
            expected_bytes: entry.bytes,
            actual_bytes: None,
            object_status: CacheObjectStatus::Missing,
            freshness_status: CacheFreshnessStatus::Missing,
        });
    }

    let mut file = File::open(&path).map_err(|source| FletchError::ReadSource {
        path: path.display().to_string(),
        source,
    })?;
    let (actual_sha256, actual_bytes) = hash_stream(&mut file, &path)?;
    let object_status = if actual_sha256 == entry.sha256 && actual_bytes == entry.bytes {
        CacheObjectStatus::Verified
    } else {
        CacheObjectStatus::HashMismatch
    };
    let freshness_status =
        if object_status == CacheObjectStatus::Verified && cache_hit_is_fresh(&path, freshness)? {
            CacheFreshnessStatus::Fresh
        } else {
            CacheFreshnessStatus::Stale
        };

    Ok(CacheStatus {
        dataset_id: entry.dataset_id.clone(),
        cache_key: entry.cache_key.clone(),
        relative_path,
        absolute_path: path.display().to_string(),
        expected_sha256: entry.sha256.clone(),
        actual_sha256: Some(actual_sha256),
        expected_bytes: entry.bytes,
        actual_bytes: Some(actual_bytes),
        object_status,
        freshness_status,
    })
}

fn cache_hit_outcome(
    plan: &FetchPlan,
    cache_key: &str,
    relative_path: String,
    destination: PathBuf,
    options: &FetchOptions,
) -> Result<FetchOutcome, FletchError> {
    let mut file = File::open(&destination).map_err(|source| FletchError::ReadSource {
        path: destination.display().to_string(),
        source,
    })?;
    let (sha256, bytes) = hash_stream(&mut file, &destination)?;
    if let Some(expected) = &options.expected_sha256 {
        if &sha256 != expected {
            return Err(FletchError::ChecksumMismatch {
                dataset_id: plan.dataset_id.clone(),
                expected: expected.clone(),
                actual: sha256,
            });
        }
    }

    Ok(FetchOutcome {
        entry: CacheEntry {
            dataset_id: plan.dataset_id.clone(),
            version: plan.version.clone(),
            source_url: plan.source.url.clone(),
            cache_key: cache_key.to_string(),
            relative_path,
            sha256,
            bytes,
            fetched_at_ms: options.fetched_at_ms.unwrap_or(0),
            verified: true,
        },
        path: destination,
    })
}

fn write_stream_to_temp<R: Read>(
    reader: &mut R,
    temp_path: &Path,
    max_bytes_per_second: Option<u64>,
) -> Result<(String, u64), FletchError> {
    let mut file = File::create(temp_path).map_err(|source| FletchError::WriteCache {
        path: temp_path.display().to_string(),
        source,
    })?;
    let mut hasher = Sha256::new();
    let mut bytes = 0u64;
    let mut buffer = [0u8; 64 * 1024];
    let started = Instant::now();

    loop {
        let read = reader
            .read(&mut buffer)
            .map_err(|source| FletchError::ReadSource {
                path: temp_path.display().to_string(),
                source,
            })?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
        file.write_all(&buffer[..read])
            .map_err(|source| FletchError::WriteCache {
                path: temp_path.display().to_string(),
                source,
            })?;
        bytes += read as u64;
        throttle_bandwidth(bytes, max_bytes_per_second, started);
    }

    file.flush().map_err(|source| FletchError::WriteCache {
        path: temp_path.display().to_string(),
        source,
    })?;
    file.sync_all().map_err(|source| FletchError::WriteCache {
        path: temp_path.display().to_string(),
        source,
    })?;

    Ok((format!("sha256:{:x}", hasher.finalize()), bytes))
}

fn hash_stream<R: Read>(reader: &mut R, path: &Path) -> Result<(String, u64), FletchError> {
    let mut hasher = Sha256::new();
    let mut bytes = 0u64;
    let mut buffer = [0u8; 64 * 1024];
    loop {
        let read = reader
            .read(&mut buffer)
            .map_err(|source| FletchError::ReadSource {
                path: path.display().to_string(),
                source,
            })?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
        bytes += read as u64;
    }
    Ok((format!("sha256:{:x}", hasher.finalize()), bytes))
}

fn throttle_bandwidth(bytes: u64, max_bytes_per_second: Option<u64>, started: Instant) {
    let Some(limit) = max_bytes_per_second else {
        return;
    };
    if limit == 0 {
        return;
    }
    let expected = Duration::from_secs_f64(bytes as f64 / limit as f64);
    let elapsed = started.elapsed();
    if expected > elapsed {
        std::thread::sleep(expected - elapsed);
    }
}

fn promote_temp(temp_path: &Path, destination: &Path) -> Result<(), FletchError> {
    if destination.exists() {
        std::fs::remove_file(destination).map_err(|source| FletchError::WriteCache {
            path: destination.display().to_string(),
            source,
        })?;
    }
    std::fs::rename(temp_path, destination).map_err(|source| FletchError::WriteCache {
        path: destination.display().to_string(),
        source,
    })
}

fn file_source_path(source_url: &str) -> PathBuf {
    source_url
        .strip_prefix("file://")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(source_url))
}

fn relative_cache_path(cache_key: &str) -> String {
    let hex = cache_key.strip_prefix("sha256:").unwrap_or(cache_key);
    format!("objects/sha256/{hex}")
}

fn source_kind_key(kind: &SourceKind) -> &'static str {
    match kind {
        SourceKind::Http => "http",
        SourceKind::File => "file",
        SourceKind::Adapter => "adapter",
    }
}

fn cache_path(cache_root: &Path, relative_path: &str) -> PathBuf {
    relative_path
        .split('/')
        .fold(cache_root.to_path_buf(), |path, part| path.join(part))
}

fn normalize_relative_cache_path(relative_path: &str) -> Result<String, FletchError> {
    let path = Path::new(relative_path);
    if path.is_absolute() {
        return Err(FletchError::UnsafeCachePath {
            relative_path: relative_path.to_string(),
        });
    }
    let mut parts = Vec::new();
    for part in relative_path.split(['/', '\\']) {
        if part.is_empty() || part == "." || part == ".." {
            return Err(FletchError::UnsafeCachePath {
                relative_path: relative_path.to_string(),
            });
        }
        parts.push(part);
    }
    if parts.is_empty() {
        return Err(FletchError::UnsafeCachePath {
            relative_path: relative_path.to_string(),
        });
    }
    Ok(parts.join("/"))
}

fn collect_prune_candidates(
    cache_root: &Path,
    dir: &Path,
    referenced: &BTreeSet<String>,
    candidates: &mut Vec<PruneCandidate>,
) -> Result<(), FletchError> {
    for entry in std::fs::read_dir(dir).map_err(|source| FletchError::ReadSource {
        path: dir.display().to_string(),
        source,
    })? {
        let entry = entry.map_err(|source| FletchError::ReadSource {
            path: dir.display().to_string(),
            source,
        })?;
        let path = entry.path();
        let metadata = entry.metadata().map_err(|source| FletchError::ReadSource {
            path: path.display().to_string(),
            source,
        })?;
        if metadata.is_dir() {
            collect_prune_candidates(cache_root, &path, referenced, candidates)?;
            continue;
        }
        if !metadata.is_file() {
            continue;
        }
        let Ok(stripped) = path.strip_prefix(cache_root) else {
            continue;
        };
        let relative_path = stripped
            .components()
            .map(|component| component.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/");
        if referenced.contains(&relative_path) {
            continue;
        }
        candidates.push(PruneCandidate {
            relative_path,
            absolute_path: path.display().to_string(),
            bytes: metadata.len(),
        });
    }
    Ok(())
}

fn read_quiver_manifest(path: &Path) -> Result<QuiverManifest, FletchError> {
    let json = std::fs::read_to_string(path).map_err(|source| FletchError::ReadSource {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&json).map_err(|source| FletchError::ReadQuiverJson {
        path: path.display().to_string(),
        source,
    })
}

fn write_quiver_manifest(path: &Path, manifest: &QuiverManifest) -> Result<(), FletchError> {
    let json =
        serde_json::to_string_pretty(manifest).map_err(|source| FletchError::WriteQuiverJson {
            path: path.display().to_string(),
            source,
        })?;
    std::fs::write(path, json).map_err(|source| FletchError::WriteCache {
        path: path.display().to_string(),
        source,
    })
}

fn copy_file(source: &Path, destination: &Path) -> Result<(), FletchError> {
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent).map_err(|source| FletchError::WriteCache {
            path: parent.display().to_string(),
            source,
        })?;
    }
    std::fs::copy(source, destination).map_err(|source| FletchError::WriteCache {
        path: destination.display().to_string(),
        source,
    })?;
    Ok(())
}

fn safe_path_label(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == '.' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn push_graph_node(nodes: &mut Vec<GraphNode>, seen_nodes: &mut BTreeSet<String>, node: GraphNode) {
    if seen_nodes.insert(node.id.clone()) {
        nodes.push(node);
    }
}

fn graph_fletch_id(dataset_id: &str) -> String {
    format!("fletch:{dataset_id}")
}

fn graph_shaft_id(cache_key: &str) -> String {
    format!("shaft:{cache_key}")
}

fn graph_ledger_id(cache_key: &str) -> String {
    format!("ledger-entry:{cache_key}")
}

fn temp_path_for(destination: &Path) -> PathBuf {
    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);
    let mut file_name = destination
        .file_name()
        .map(|name| name.to_os_string())
        .unwrap_or_else(|| "fletch-cache".into());
    let unique = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    file_name.push(format!(".{}.{}.tmp", std::process::id(), unique));
    destination.with_file_name(file_name)
}

fn now_ms() -> Result<u64, FletchError> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| FletchError::ClockBeforeEpoch)?;
    Ok(duration.as_millis() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_plan_has_stable_schema_and_cache_key() {
        let plan = fetch_plan("nhl:season:1993", "https://example.test/1993.json").unwrap();

        assert_eq!(plan.schema_version, FLETCH_PLAN_SCHEMA);
        assert_eq!(plan.cache_policy, CachePolicy::default());
        assert!(cache_key(&plan).starts_with("sha256:"));
        assert_eq!(cache_key(&plan).len(), 71);
    }

    #[test]
    fn fetch_plan_rejects_empty_inputs() {
        assert!(matches!(
            fetch_plan("", "https://example.test"),
            Err(FletchError::EmptyDatasetId)
        ));
        assert!(matches!(
            fetch_plan("route:tiles", ""),
            Err(FletchError::EmptySourceUrl)
        ));
    }

    #[test]
    fn cache_manifest_rejects_invalid_hashes() {
        let entry = CacheEntry {
            dataset_id: "census:2020:tracts".to_string(),
            version: Some("2020".to_string()),
            source_url: "https://example.test/tracts.zip".to_string(),
            cache_key: "sha256:abc".to_string(),
            relative_path: "census/2020/tracts.zip".to_string(),
            sha256: "abc".to_string(),
            bytes: 42,
            fetched_at_ms: 0,
            verified: false,
        };

        assert!(matches!(
            cache_manifest(".fletch/cache", vec![entry]),
            Err(FletchError::InvalidSha256 { .. })
        ));
    }

    #[test]
    fn file_fetch_promotes_temp_and_returns_verified_entry() {
        let root = unique_temp_dir("file-fetch");
        let source = root.join("source.json");
        let cache_root = root.join("cache");
        std::fs::write(&source, br#"{"ok":true}"#).unwrap();
        let plan =
            fetch_plan_with_kind("test:file", source.display().to_string(), SourceKind::File)
                .unwrap();

        let outcome = fetch_to_cache(
            &plan,
            FetchOptions::new(&cache_root).with_fetched_at_ms(123),
        )
        .unwrap();

        assert_eq!(std::fs::read(&outcome.path).unwrap(), br#"{"ok":true}"#);
        assert_eq!(outcome.entry.dataset_id, "test:file");
        assert_eq!(outcome.entry.bytes, 11);
        assert_eq!(outcome.entry.fetched_at_ms, 123);
        assert!(outcome.entry.verified);
        assert!(outcome.entry.sha256.starts_with("sha256:"));
        assert!(std::fs::read_dir(outcome.path.parent().unwrap())
            .unwrap()
            .all(|entry| !entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .contains(".tmp")));

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn file_fetch_rejects_checksum_mismatch_and_cleans_temp() {
        let root = unique_temp_dir("checksum-mismatch");
        let source = root.join("source.txt");
        let cache_root = root.join("cache");
        std::fs::write(&source, b"hello").unwrap();
        let plan =
            fetch_plan_with_kind("test:file", source.display().to_string(), SourceKind::File)
                .unwrap();

        let result = fetch_to_cache(
            &plan,
            FetchOptions::new(&cache_root).with_expected_sha256(format!("sha256:{:064}", 0)),
        );

        assert!(matches!(result, Err(FletchError::ChecksumMismatch { .. })));
        let objects = cache_root.join("objects").join("sha256");
        if objects.exists() {
            assert!(std::fs::read_dir(objects).unwrap().all(|entry| !entry
                .unwrap()
                .file_name()
                .to_string_lossy()
                .contains(".tmp")));
        }

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn fetch_rejects_zero_bandwidth_limit() {
        let root = unique_temp_dir("zero-bandwidth");
        let source = root.join("source.txt");
        std::fs::write(&source, b"hello").unwrap();
        let plan =
            fetch_plan_with_kind("test:file", source.display().to_string(), SourceKind::File)
                .unwrap();

        let result = fetch_to_cache(
            &plan,
            FetchOptions::new(root.join("cache")).with_max_bytes_per_second(0),
        );

        assert!(matches!(result, Err(FletchError::InvalidBandwidthLimit)));

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn inspect_cache_manifest_reports_verified_and_missing_entries() {
        let root = unique_temp_dir("inspect-cache");
        let source = root.join("source.txt");
        let cache_root = root.join("cache");
        std::fs::write(&source, b"hello").unwrap();
        let plan =
            fetch_plan_with_kind("test:file", source.display().to_string(), SourceKind::File)
                .unwrap();
        let outcome = fetch_to_cache(
            &plan,
            FetchOptions::new(&cache_root).with_fetched_at_ms(123),
        )
        .unwrap();
        let mut missing = outcome.entry.clone();
        missing.dataset_id = "test:missing".to_string();
        missing.relative_path = "objects/sha256/missing".to_string();
        let manifest = cache_manifest(
            cache_root.display().to_string(),
            vec![outcome.entry.clone(), missing],
        )
        .unwrap();

        let statuses = inspect_cache_manifest(&manifest, &FreshnessPolicy::Immutable).unwrap();

        assert_eq!(statuses.len(), 2);
        assert_eq!(statuses[0].object_status, CacheObjectStatus::Verified);
        assert_eq!(statuses[0].freshness_status, CacheFreshnessStatus::Fresh);
        assert_eq!(statuses[0].actual_bytes, Some(5));
        assert_eq!(statuses[1].object_status, CacheObjectStatus::Missing);
        assert_eq!(statuses[1].freshness_status, CacheFreshnessStatus::Missing);

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn plan_cache_prune_reports_unreferenced_objects_without_deleting() {
        let root = unique_temp_dir("prune-cache");
        let source = root.join("source.txt");
        let cache_root = root.join("cache");
        std::fs::write(&source, b"hello").unwrap();
        let plan =
            fetch_plan_with_kind("test:file", source.display().to_string(), SourceKind::File)
                .unwrap();
        let outcome = fetch_to_cache(&plan, FetchOptions::new(&cache_root)).unwrap();
        let orphan = cache_root.join("objects").join("sha256").join("orphan");
        std::fs::write(&orphan, b"orphan").unwrap();
        let manifest =
            cache_manifest(cache_root.display().to_string(), vec![outcome.entry]).unwrap();

        let prune = plan_cache_prune(&manifest).unwrap();

        assert_eq!(prune.keep_count, 1);
        assert_eq!(prune.prune_count, 1);
        assert_eq!(prune.prune_bytes, 6);
        assert_eq!(prune.candidates[0].relative_path, "objects/sha256/orphan");
        assert!(orphan.exists());

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn quiver_export_and_import_stage_verified_entries() {
        let root = unique_temp_dir("quiver");
        let source = root.join("source.txt");
        let cache_root = root.join("cache");
        let quiver_root = root.join("quiver");
        let import_cache_root = root.join("import-cache");
        std::fs::write(&source, b"hello quiver").unwrap();
        let plan = fetch_plan_with_kind(
            "test:quiver",
            source.display().to_string(),
            SourceKind::File,
        )
        .unwrap();
        let outcome = fetch_to_cache(&plan, FetchOptions::new(&cache_root)).unwrap();
        let manifest =
            cache_manifest(cache_root.display().to_string(), vec![outcome.entry]).unwrap();

        let exported = export_quiver(&manifest, "test:quiver-pack", &quiver_root).unwrap();
        let imported = import_quiver(&quiver_root, &import_cache_root).unwrap();
        let statuses =
            inspect_cache_manifest(&imported.staged_manifest, &FreshnessPolicy::Immutable).unwrap();

        assert_eq!(exported.manifest.schema_version, FLETCH_QUIVER_SCHEMA);
        assert_eq!(exported.manifest.quiver_id, "test:quiver-pack");
        assert!(exported.path.exists());
        assert_eq!(
            imported.stage_root,
            import_cache_root
                .join("staged")
                .join("quivers")
                .join("test_quiver-pack")
        );
        assert_eq!(statuses[0].object_status, CacheObjectStatus::Verified);
        assert!(!import_cache_root.join("objects").exists());

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn graph_from_manifest_exports_fletch_shaft_and_ledger_nodes() {
        let root = unique_temp_dir("graph");
        let source = root.join("source.txt");
        let cache_root = root.join("cache");
        std::fs::write(&source, b"hello graph").unwrap();
        let plan =
            fetch_plan_with_kind("test:graph", source.display().to_string(), SourceKind::File)
                .unwrap();
        let outcome = fetch_to_cache(&plan, FetchOptions::new(&cache_root)).unwrap();
        let manifest =
            cache_manifest(cache_root.display().to_string(), vec![outcome.entry]).unwrap();

        let graph = graph_from_manifest(&manifest);

        assert_eq!(graph.schema_version, FLETCH_GRAPH_SCHEMA);
        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.edges.len(), 2);
        assert!(graph
            .nodes
            .iter()
            .any(|node| node.id == "fletch:test:graph" && node.kind == GraphNodeKind::Fletch));
        assert!(graph
            .edges
            .iter()
            .any(|edge| edge.kind == GraphEdgeKind::SatisfiedBy));
        assert!(graph
            .edges
            .iter()
            .any(|edge| edge.kind == GraphEdgeKind::Documents));

        let _ = std::fs::remove_dir_all(root);
    }

    fn unique_temp_dir(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "fletch-{label}-{}-{}",
            std::process::id(),
            now_ms().unwrap()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }
}
