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
    local_url_map, mdcrop_index_from_manifest, mdloom_document_manifest, offline_cache_report,
    partition_invalidation_report, partition_state_from_manifest, plan_cache_prune,
    preview_archive_expansion, preview_manifest_merge, preview_rollback, preview_rollup_edges,
    publish_report_from_manifest, publisher_bundle_report, quiver_merge_ready_report,
    read_cache_manifest_json, registry_index_from_registries, search_registry_index,
    slice_active_partition_set, slice_adapter_source_report, slice_archive_expansion_preview,
    slice_cache_index_report, slice_local_url_map, slice_mdcrop_index_report,
    slice_mdloom_document_manifest, slice_partition_state, slice_quiver_merge_ready_report,
    slice_registry_validation_report, summarize_cache_manifest, summarize_quiver,
    tips_from_manifest, upsert_cache_manifest_entries, validate_registry, verify_cache_manifest,
    verify_quiver_bundle, write_cache_manifest_json, AdapterHandoffReport, AliasState, CacheEntry,
    CacheIndexGatePolicy, CacheIndexReport, CacheManifest, FetchOptions, FetchPlan, FletchRegistry,
    FreshnessPolicy, LabelState, LocalUrlMap, MdcropIndexReport, MdloomDocumentManifest,
    PartitionState, QuiverManifest, QuiverSummary, RegistryIndexReport, RegistryIndexRow,
    RollupPreview, SourceKind,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub(crate) enum PartitionCommands {
    /// Emit fletch.partition-state.v1 rows from a cache manifest.
    State {
        /// Path to a fletch.cache-manifest.v1 JSON file.
        #[arg(long)]
        manifest: PathBuf,
        /// Optional product-neutral group id assigned to every emitted row.
        #[arg(long)]
        group_id: Option<String>,
        /// Number of partition rows to skip before output.
        #[arg(long, default_value_t = 0)]
        offset: usize,
        /// Maximum number of partition rows to output.
        #[arg(long)]
        limit: Option<usize>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Preview parent/child rollup edges over partition state.
    RollupPreview {
        /// Path to a fletch.partition-state.v1 JSON file.
        #[arg(long)]
        partition_state: PathBuf,
        /// Product-neutral rollup id.
        #[arg(long)]
        rollup_id: String,
        /// Child partition id to include. Repeat for a subset; omit to include all.
        #[arg(long = "child-partition-id")]
        child_partition_ids: Vec<String>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Report stale, folded, and superseded partition metadata.
    InvalidationReport {
        /// Path to a fletch.partition-state.v1 JSON file.
        #[arg(long)]
        partition_state: PathBuf,
        /// Partition id to mark stale. Repeat for multiple partitions.
        #[arg(long = "stale-partition-id")]
        stale_partition_ids: Vec<String>,
        /// Partition id to mark folded. Repeat for multiple partitions.
        #[arg(long = "folded-partition-id")]
        folded_partition_ids: Vec<String>,
        /// Partition id to mark superseded. Repeat for multiple partitions.
        #[arg(long = "superseded-partition-id")]
        superseded_partition_ids: Vec<String>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Emit query-facing active partition rows from alias, label, and rollup evidence.
    ActiveSet {
        /// Path to a fletch.partition-state.v1 JSON file.
        #[arg(long)]
        partition_state: PathBuf,
        /// Optional fletch.alias-state.v1 JSON file.
        #[arg(long)]
        alias_state: Option<PathBuf>,
        /// Optional fletch.label-state.v1 JSON file.
        #[arg(long)]
        label_state: Option<PathBuf>,
        /// Optional fletch.rollup-preview.v1 JSON file.
        #[arg(long)]
        rollup_preview: Option<PathBuf>,
        /// Filter active rows: true for active, false for inactive.
        #[arg(long)]
        active: Option<bool>,
        /// Number of active-set rows to skip before output.
        #[arg(long, default_value_t = 0)]
        offset: usize,
        /// Maximum number of active-set rows to output.
        #[arg(long)]
        limit: Option<usize>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum CacheCommands {
    /// Emit a compact fletch.cache-index.v1 report from a cache manifest.
    Index {
        /// Path to a fletch.cache-manifest.v1 JSON file.
        #[arg(long)]
        manifest: PathBuf,
        /// Optional exact dataset id lookup.
        #[arg(long)]
        dataset_id: Option<String>,
        /// Optional exact cache key lookup.
        #[arg(long)]
        cache_key: Option<String>,
        /// Optional verified flag filter.
        #[arg(long)]
        verified: Option<bool>,
        /// Number of index rows to skip before output.
        #[arg(long, default_value_t = 0)]
        offset: usize,
        /// Maximum number of index rows to output.
        #[arg(long)]
        limit: Option<usize>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Gate a cache index against product-supplied expected dataset ids.
    IndexGate {
        /// Path to a fletch.cache-manifest.v1 JSON file.
        #[arg(long)]
        manifest: PathBuf,
        /// FLETCH registry whose HTTP/file fletch IDs should be expected dataset IDs.
        #[arg(long = "expected-registry", value_name = "FILE")]
        expected_registries: Vec<PathBuf>,
        /// Expected dataset id. Repeat for ROUTE/BISECT/ICELINES-owned sets.
        #[arg(long = "expected-dataset-id")]
        expected_dataset_ids: Vec<String>,
        /// Fail if an expected dataset id is absent from the index.
        #[arg(long)]
        require_all_expected: bool,
        /// Allow unverified entries to pass the gate.
        #[arg(long)]
        allow_unverified: bool,
        /// Exit non-zero when the generated gate report does not pass.
        #[arg(long)]
        gate: bool,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Compare two fletch.cache-index.v1 reports without reading object bytes.
    IndexDiff {
        /// Path to the base fletch.cache-index.v1 JSON file.
        #[arg(long)]
        base_index: PathBuf,
        /// Path to the candidate fletch.cache-index.v1 JSON file.
        #[arg(long)]
        candidate_index: PathBuf,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// List ledger entries from a cache manifest.
    List {
        /// Path to a fletch.cache-manifest.v1 JSON file.
        #[arg(long)]
        manifest: PathBuf,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Verify cached objects against ledger hashes and byte counts.
    Verify {
        /// Path to a fletch.cache-manifest.v1 JSON file.
        #[arg(long)]
        manifest: PathBuf,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Report fresh, stale, missing, or corrupt cache state.
    Status {
        /// Path to a fletch.cache-manifest.v1 JSON file.
        #[arg(long)]
        manifest: PathBuf,
        /// Freshness policy to evaluate.
        #[arg(long, value_enum, default_value_t = CliFreshness::Immutable)]
        freshness: CliFreshness,
        /// Max age in days when --freshness max-age-days is used.
        #[arg(long)]
        max_age_days: Option<u32>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Summarize aggregate cache health for a manifest.
    Summary {
        /// Path to a fletch.cache-manifest.v1 JSON file.
        #[arg(long)]
        manifest: PathBuf,
        /// Freshness policy to evaluate.
        #[arg(long, value_enum, default_value_t = CliFreshness::Immutable)]
        freshness: CliFreshness,
        /// Max age in days when --freshness max-age-days is used.
        #[arg(long)]
        max_age_days: Option<u32>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Report offline readiness without touching live sources.
    OfflineReport {
        /// Path to a fletch.cache-manifest.v1 JSON file.
        #[arg(long)]
        manifest: PathBuf,
        /// Freshness policy to evaluate.
        #[arg(long, value_enum, default_value_t = CliFreshness::Immutable)]
        freshness: CliFreshness,
        /// Max age in days when --freshness max-age-days is used.
        #[arg(long)]
        max_age_days: Option<u32>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Plan deletion candidates not referenced by the manifest.
    Prune {
        /// Path to a fletch.cache-manifest.v1 JSON file.
        #[arg(long)]
        manifest: PathBuf,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum QuiverCommands {
    /// Export a manifest-backed fletch.quiver.v1 directory.
    Export {
        /// Path to a fletch.cache-manifest.v1 JSON file.
        #[arg(long)]
        manifest: PathBuf,
        /// Quiver id to write into quiver.json.
        #[arg(long)]
        quiver_id: String,
        /// Output directory for the quiver.
        #[arg(long)]
        output_dir: PathBuf,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Import a fletch.quiver.v1 directory into a staged cache location.
    Import {
        /// Directory containing quiver.json and bundled objects.
        #[arg(long)]
        quiver_dir: PathBuf,
        /// Cache root where the quiver should be staged.
        #[arg(long, default_value = ".fletch/cache")]
        cache_root: PathBuf,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Summarize a fletch.quiver.v1 manifest without importing it.
    Summary {
        /// Path to a fletch.quiver.v1 JSON file.
        #[arg(long)]
        quiver: PathBuf,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Verify bundled quiver members without importing them.
    Verify {
        /// Directory containing quiver.json and bundled objects.
        #[arg(long)]
        quiver_dir: PathBuf,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Export fletch.graph.v1 nodes and edges from a quiver manifest.
    Graph {
        /// Path to a fletch.quiver.v1 JSON file.
        #[arg(long)]
        quiver: PathBuf,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Describe quiver members as merge/alias candidates without activating them.
    MergeReady {
        /// Path to a fletch.quiver.v1 JSON file.
        #[arg(long)]
        quiver: PathBuf,
        /// Optional alias id to propose for every candidate row.
        #[arg(long)]
        alias_id: Option<String>,
        /// Optional candidate status filter, such as ready or blocked-unverified.
        #[arg(long)]
        status: Option<String>,
        /// Number of candidate rows to skip before output.
        #[arg(long, default_value_t = 0)]
        offset: usize,
        /// Maximum number of candidate rows to output.
        #[arg(long)]
        limit: Option<usize>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum GraphCommands {
    /// Export fletch.graph.v1 nodes and edges from a cache manifest.
    Export {
        /// Path to a fletch.cache-manifest.v1 JSON file.
        #[arg(long)]
        manifest: PathBuf,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum RegistryCommands {
    /// Build a searchable index from one or more fletch.registry.v1 files.
    Index {
        /// Path to a fletch.registry.v1 JSON file. Repeat for multiple registries.
        #[arg(long = "file", required = true)]
        files: Vec<PathBuf>,
        /// Follow repo-registry bridge rows to remote or local registry JSON files.
        #[arg(long)]
        follow: bool,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Search a fletch.registry-index.v1 report by tag, metadata, URL, or text.
    Search {
        /// Path to a fletch.registry-index.v1 JSON file.
        #[arg(long)]
        index: PathBuf,
        /// Required tag. Repeat to require multiple tags.
        #[arg(long = "tag")]
        tags: Vec<String>,
        /// Metadata equality filter as key=value. Repeat for multiple filters.
        #[arg(long = "metadata")]
        metadata: Vec<String>,
        /// Case-insensitive text search over IDs, URLs, tags, and metadata.
        #[arg(long)]
        text: Option<String>,
        /// Number of matching rows to skip before output.
        #[arg(long, default_value_t = 0)]
        offset: usize,
        /// Maximum number of matching rows to output.
        #[arg(long)]
        limit: Option<usize>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Serve a local browser UI for searching a fletch.registry-index.v1 report.
    Web {
        /// Path to an existing fletch.registry-index.v1 JSON file.
        #[arg(long)]
        index: Option<PathBuf>,
        /// Path to a fletch.registry.v1 JSON file. Repeat for multiple registries.
        #[arg(long = "file")]
        files: Vec<PathBuf>,
        /// Follow repo-registry bridge rows when building an in-memory index from --file inputs.
        #[arg(long)]
        follow: bool,
        /// Host interface to bind.
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        /// Port to bind. Use 0 to ask the OS for an available port.
        #[arg(long, default_value_t = 7878)]
        port: u16,
        /// Open the local registry browser URL in the default browser after binding.
        #[arg(long)]
        open: bool,
    },
    /// Export graph JSON from a fletch.registry.v1 file.
    Graph {
        /// Path to a fletch.registry.v1 JSON file.
        #[arg(long)]
        file: PathBuf,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Resolve registered fletches into a fletch.flight.v1 dry-run plan.
    Flight {
        /// Path to a fletch.registry.v1 JSON file.
        #[arg(long)]
        file: PathBuf,
        /// Fletch id to resolve. Repeat to request multiple roots; omit for all.
        #[arg(long = "fletch-id")]
        fletch_ids: Vec<String>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Report product-neutral source rows from a registry.
    AdapterSources {
        /// Path to a fletch.registry.v1 JSON file.
        #[arg(long)]
        file: PathBuf,
        /// Filter source rows by adapter-owned status.
        #[arg(long)]
        adapter_owned: Option<bool>,
        /// Number of source rows to skip before output.
        #[arg(long, default_value_t = 0)]
        offset: usize,
        /// Maximum number of source rows to output.
        #[arg(long)]
        limit: Option<usize>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Validate registry structure and adapter-source declarations.
    Validate {
        /// Path to a fletch.registry.v1 JSON file.
        #[arg(long)]
        file: PathBuf,
        /// Optional finding severity filter, such as error or warning.
        #[arg(long)]
        severity: Option<String>,
        /// Number of validation findings to skip before output.
        #[arg(long, default_value_t = 0)]
        offset: usize,
        /// Maximum number of validation findings to output.
        #[arg(long)]
        limit: Option<usize>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Preview archive expansion edges without extracting archive contents.
    ArchivePreview {
        /// Path to a fletch.registry.v1 JSON file.
        #[arg(long)]
        file: PathBuf,
        /// Fletch id for the archive/source that expands to children.
        #[arg(long)]
        archive_fletch_id: String,
        /// Number of child rows to skip before output.
        #[arg(long, default_value_t = 0)]
        offset: usize,
        /// Maximum number of child rows to output.
        #[arg(long)]
        limit: Option<usize>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Summarize adapter-owned registry, graph, and flight handoff inputs.
    Handoff {
        /// Path to a fletch.registry.v1 JSON file.
        #[arg(long)]
        file: PathBuf,
        /// Fletch id to resolve in the handoff flight. Repeat for multiple roots.
        #[arg(long = "fletch-id")]
        fletch_ids: Vec<String>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum TipCommands {
    /// Generate lightweight tips from cached objects referenced by a manifest.
    FromManifest {
        /// Path to a fletch.cache-manifest.v1 JSON file.
        #[arg(long)]
        manifest: PathBuf,
        /// Maximum bytes to sample from each cached object.
        #[arg(long, default_value_t = 4096)]
        max_bytes: usize,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum PublishCommands {
    /// Generate a publish-ready report from a cache manifest.
    FromManifest {
        /// Path to a fletch.cache-manifest.v1 JSON file.
        #[arg(long)]
        manifest: PathBuf,
        /// Freshness policy to evaluate.
        #[arg(long, value_enum, default_value_t = CliFreshness::Immutable)]
        freshness: CliFreshness,
        /// Max age in days when --freshness max-age-days is used.
        #[arg(long)]
        max_age_days: Option<u32>,
        /// Maximum bytes to sample from each cached object for tips.
        #[arg(long, default_value_t = 4096)]
        max_tip_bytes: usize,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Generate a MDCROP-indexable report from a cache manifest.
    MdcropIndex {
        /// Path to a fletch.cache-manifest.v1 JSON file.
        #[arg(long)]
        manifest: PathBuf,
        /// Freshness policy to evaluate.
        #[arg(long, value_enum, default_value_t = CliFreshness::Immutable)]
        freshness: CliFreshness,
        /// Max age in days when --freshness max-age-days is used.
        #[arg(long)]
        max_age_days: Option<u32>,
        /// Maximum bytes to sample from each cached object for tips.
        #[arg(long, default_value_t = 4096)]
        max_tip_bytes: usize,
        /// Optional row type filter, such as cache-status, graph-node, graph-edge, or tip.
        #[arg(long)]
        row_type: Option<String>,
        /// Number of matching rows to skip before output.
        #[arg(long, default_value_t = 0)]
        offset: usize,
        /// Maximum number of matching rows to output.
        #[arg(long)]
        limit: Option<usize>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Generate MDLOOM document anchors from a MDCROP index report.
    ProofDocs {
        /// Path to a fletch.mdcrop-index.v1 JSON file.
        #[arg(long)]
        mdcrop_index: PathBuf,
        /// Number of generated document anchors to skip before output.
        #[arg(long, default_value_t = 0)]
        offset: usize,
        /// Maximum number of generated document anchors to output.
        #[arg(long)]
        limit: Option<usize>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Generate stable local URLs from a MDLOOM document manifest.
    LocalUrlMap {
        /// Path to a fletch.mdloom-docs.v1 JSON file.
        #[arg(long)]
        mdloom_docs: PathBuf,
        /// Local base path or URL prefix for generated documents.
        #[arg(long, default_value = "fletch")]
        base_path: String,
        /// Number of generated URL entries to skip before output.
        #[arg(long, default_value_t = 0)]
        offset: usize,
        /// Maximum number of generated URL entries to output.
        #[arg(long)]
        limit: Option<usize>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Summarize publisher inputs for downstream MDCROP/MDLOOM backends.
    Bundle {
        /// Path to a fletch.mdcrop-index.v1 JSON file.
        #[arg(long)]
        mdcrop_index: PathBuf,
        /// Path to a fletch.mdloom-docs.v1 JSON file.
        #[arg(long)]
        mdloom_docs: PathBuf,
        /// Path to a fletch.local-url-map.v1 JSON file.
        #[arg(long)]
        local_url_map: PathBuf,
        /// Optional fletch.quiver-summary.v1 JSON file.
        #[arg(long)]
        quiver_summary: Option<PathBuf>,
        /// Optional fletch.adapter-handoff.v1 JSON file.
        #[arg(long)]
        adapter_handoff: Option<PathBuf>,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum MergeCommands {
    /// Preview candidate ledger changes against an active ledger.
    Preview {
        /// Current active fletch.cache-manifest.v1 JSON file.
        #[arg(long)]
        active: PathBuf,
        /// Candidate fletch.cache-manifest.v1 JSON file.
        #[arg(long)]
        candidate: PathBuf,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Emit an active alias state pointing at a manifest entry.
    AliasState {
        /// Source fletch.cache-manifest.v1 JSON file.
        #[arg(long)]
        manifest: PathBuf,
        /// Product-neutral alias id, e.g. current or stable.
        #[arg(long)]
        alias_id: String,
        /// Dataset id in the manifest that the alias should point at.
        #[arg(long)]
        dataset_id: String,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Emit labels and optional pins over an alias state.
    LabelState {
        /// Source fletch.alias-state.v1 JSON file.
        #[arg(long)]
        alias_state: PathBuf,
        /// Label id to apply to every alias in the alias state.
        #[arg(long)]
        label_id: String,
        /// Pin the label to the current alias targets.
        #[arg(long)]
        pin: bool,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Preview restoring aliases to a prior label state without mutation.
    RollbackPreview {
        /// Current fletch.alias-state.v1 JSON file.
        #[arg(long)]
        alias_state: PathBuf,
        /// Target fletch.label-state.v1 JSON file.
        #[arg(long)]
        label_state: PathBuf,
        /// Optional JSON output path. Defaults to stdout.
        #[arg(long)]
        output: Option<PathBuf>,
    },
}

impl From<CliSourceKind> for SourceKind {
    fn from(value: CliSourceKind) -> Self {
        match value {
            CliSourceKind::Http => Self::Http,
            CliSourceKind::File => Self::File,
        }
    }
}

