//! Split from main.rs (ROUTE-style layout).
#![allow(unused_imports, dead_code, unused_variables)]
use crate::*;
use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use fletch_core::{
    active_partition_set, adapter_handoff_report, adapter_sources_from_registry,
    alias_state_from_manifest, cache_index_diff, cache_index_from_manifest,
    cache_index_gate_report, cache_key, cache_list, cache_manifest, dry_run_flight, export_quiver,
    fetch_plan, fetch_plan_with_kind, fetch_to_cache, graph_from_manifest, graph_from_quiver,
    graph_from_registry, import_quiver, inspect_cache_manifest, label_state_from_aliases,
    local_url_map, mdcrop_index_from_manifest, offline_cache_report, partition_invalidation_report,
    partition_state_from_manifest, plan_cache_prune, preview_archive_expansion,
    preview_manifest_merge, preview_rollback, preview_rollup_edges, proof_document_manifest,
    publish_report_from_manifest, publisher_bundle_report, quiver_merge_ready_report,
    read_cache_manifest_json, registry_index_from_registries, search_registry_index,
    slice_active_partition_set, slice_adapter_source_report, slice_archive_expansion_preview,
    slice_cache_index_report, slice_local_url_map, slice_mdcrop_index_report,
    slice_partition_state, slice_proof_document_manifest, slice_quiver_merge_ready_report,
    slice_registry_validation_report, summarize_cache_manifest, summarize_quiver,
    tips_from_manifest, upsert_cache_manifest_entries, validate_registry, verify_cache_manifest,
    verify_quiver_bundle, write_cache_manifest_json, AdapterHandoffReport, AliasState, CacheEntry,
    CacheIndexGatePolicy, CacheIndexReport, CacheManifest, FetchOptions, FetchPlan, FletchRegistry,
    FreshnessPolicy, LabelState, LocalUrlMap, MdcropIndexReport, PartitionState,
    ProofDocumentManifest, QuiverManifest, QuiverSummary, RegistryIndexReport, RegistryIndexRow,
    RollupPreview, SourceKind,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "fletch")]
#[command(about = "Fetch/cache manifests for reproducible data pipelines")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// Emit a fletch.plan.v1 fetch plan.
    Plan {
        /// Logical dataset id, e.g. nhl:season:1993 or census:2020:tracts.
        #[arg(long)]
        dataset_id: String,
        /// Source URL for the initial generic HTTP source.
        #[arg(long)]
        url: String,
        /// Source kind for the shaft.
        #[arg(long, value_enum, default_value_t = CliSourceKind::Http)]
        source_kind: CliSourceKind,
        /// Generic HTTP header as name=value. Repeat for multiple headers.
        #[arg(long = "header")]
        headers: Vec<String>,
        /// Freshness policy for this fletch.
        #[arg(long, value_enum, default_value_t = CliFreshness::Immutable)]
        freshness: CliFreshness,
        /// Max age in days when --freshness max-age-days is used.
        #[arg(long)]
        max_age_days: Option<u32>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Emit the deterministic cache key for a dataset URL pair.
    Key {
        /// Logical dataset id.
        #[arg(long)]
        dataset_id: String,
        /// Source URL.
        #[arg(long)]
        url: String,
        /// Generic HTTP header as name=value. Repeat for multiple headers.
        #[arg(long = "header")]
        headers: Vec<String>,
    },
    /// Fetch a HTTP/file shaft into a cache root and emit a manifest.
    Fetch {
        /// Logical dataset id.
        #[arg(long)]
        dataset_id: String,
        /// Source URL or file path.
        #[arg(long)]
        url: String,
        /// Source kind for the shaft.
        #[arg(long, value_enum, default_value_t = CliSourceKind::Http)]
        source_kind: CliSourceKind,
        /// Generic HTTP header as name=value. Repeat for multiple headers.
        #[arg(long = "header")]
        headers: Vec<String>,
        /// Cache root. Defaults to .fletch/cache.
        #[arg(long, default_value = ".fletch/cache")]
        cache_root: PathBuf,
        /// Expected sha256, formatted as sha256:<64 lowercase hex chars>.
        #[arg(long)]
        expect_sha256: Option<String>,
        /// Prior manifest whose matching ledger entry can verify cache hits.
        #[arg(long)]
        trusted_manifest: Option<PathBuf>,
        /// Maximum transfer/write rate in bytes per second.
        #[arg(long)]
        max_bytes_per_second: Option<u64>,
        /// Request timeout in milliseconds for generic HTTP fetches.
        #[arg(long)]
        timeout_ms: Option<u64>,
        /// Retry attempts after the initial generic fetch attempt fails.
        #[arg(long, default_value_t = 0)]
        retry_attempts: u32,
        /// Re-fetch even if the cache policy says the existing object is fresh.
        #[arg(long)]
        force: bool,
        /// Do not fetch live data; return an error if no fresh cache hit exists.
        #[arg(long)]
        offline: bool,
        /// Freshness policy for this fletch.
        #[arg(long, value_enum, default_value_t = CliFreshness::Immutable)]
        freshness: CliFreshness,
        /// Max age in days when --freshness max-age-days is used.
        #[arg(long)]
        max_age_days: Option<u32>,
        /// Optional JSON manifest output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Fetch using a saved fletch.plan.v1 file.
    FetchPlan {
        /// Path to a fletch.plan.v1 JSON file.
        #[arg(long)]
        plan: PathBuf,
        /// Cache root. Defaults to .fletch/cache.
        #[arg(long, default_value = ".fletch/cache")]
        cache_root: PathBuf,
        /// Expected sha256, formatted as sha256:<64 lowercase hex chars>.
        #[arg(long)]
        expect_sha256: Option<String>,
        /// Prior manifest whose matching ledger entry can verify cache hits.
        #[arg(long)]
        trusted_manifest: Option<PathBuf>,
        /// Maximum transfer/write rate in bytes per second.
        #[arg(long)]
        max_bytes_per_second: Option<u64>,
        /// Request timeout in milliseconds for generic HTTP fetches.
        #[arg(long)]
        timeout_ms: Option<u64>,
        /// Retry attempts after the initial generic fetch attempt fails.
        #[arg(long, default_value_t = 0)]
        retry_attempts: u32,
        /// Re-fetch even if the cache policy says the existing object is fresh.
        #[arg(long)]
        force: bool,
        /// Do not fetch live data; return an error if no fresh cache hit exists.
        #[arg(long)]
        offline: bool,
        /// Optional JSON manifest output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Inspect and verify cached ledger entries.
    Cache {
        #[command(subcommand)]
        command: CacheCommands,
    },
    /// Export and stage-import portable quiver bundles.
    Quiver {
        #[command(subcommand)]
        command: QuiverCommands,
    },
    /// Export typed FLETCH graph JSON.
    Graph {
        #[command(subcommand)]
        command: GraphCommands,
    },
    /// Read fletch.registry.v1 definitions.
    Registry {
        #[command(subcommand)]
        command: RegistryCommands,
    },
    /// Emit lightweight fletch.tip.v1 previews.
    Tip {
        #[command(subcommand)]
        command: TipCommands,
    },
    /// Emit fletch.publish.v1 status/graph/tip reports.
    Publish {
        #[command(subcommand)]
        command: PublishCommands,
    },
    /// Preview merge/activation decisions without mutating active state.
    Merge {
        #[command(subcommand)]
        command: MergeCommands,
    },
    /// Emit partition and rollup state without mutating cache or active views.
    Partition {
        #[command(subcommand)]
        command: PartitionCommands,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum CliSourceKind {
    Http,
    File,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum CliFreshness {
    Immutable,
    MaxAgeDays,
    AlwaysCheck,
}
