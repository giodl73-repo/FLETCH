use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use thiserror::Error;

pub const FLETCH_PLAN_SCHEMA: &str = "fletch.plan.v1";
pub const FLETCH_MANIFEST_SCHEMA: &str = "fletch.cache-manifest.v1";

#[derive(Debug, Error)]
pub enum FletchError {
    #[error("[PLAN] dataset id must not be empty")]
    EmptyDatasetId,
    #[error("[PLAN] source URL must not be empty")]
    EmptySourceUrl,
    #[error("[CACHE] cache entry {dataset_id} has invalid sha256: {sha256}")]
    InvalidSha256 { dataset_id: String, sha256: String },
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

pub fn fetch_plan(
    dataset_id: impl Into<String>,
    source_url: impl Into<String>,
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
            kind: SourceKind::Http,
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
}
