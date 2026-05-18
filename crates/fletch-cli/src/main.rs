use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use fletch_core::{
    active_partition_set, adapter_handoff_report, adapter_sources_from_registry,
    alias_state_from_manifest, cache_index_diff, cache_index_from_manifest,
    cache_index_gate_report, cache_key, cache_list, cache_manifest, crop_index_from_manifest,
    dry_run_flight, export_quiver, fetch_plan, fetch_plan_with_kind, fetch_to_cache,
    graph_from_manifest, graph_from_quiver, graph_from_registry, import_quiver,
    inspect_cache_manifest, label_state_from_aliases, local_url_map, offline_cache_report,
    partition_invalidation_report, partition_state_from_manifest, plan_cache_prune,
    preview_archive_expansion, preview_manifest_merge, preview_rollback, preview_rollup_edges,
    proof_document_manifest, publish_report_from_manifest, publisher_bundle_report,
    quiver_merge_ready_report, read_cache_manifest_json, registry_index_from_registries,
    search_registry_index, slice_active_partition_set, slice_adapter_source_report,
    slice_archive_expansion_preview, slice_cache_index_report, slice_crop_index_report,
    slice_local_url_map, slice_partition_state, slice_proof_document_manifest,
    slice_quiver_merge_ready_report, slice_registry_validation_report, summarize_cache_manifest,
    summarize_quiver, tips_from_manifest, upsert_cache_manifest_entries, validate_registry,
    verify_cache_manifest, verify_quiver_bundle, write_cache_manifest_json, AdapterHandoffReport,
    AliasState, CacheEntry, CacheIndexGatePolicy, CacheIndexReport, CacheManifest, CropIndexReport,
    FetchOptions, FetchPlan, FletchRegistry, FreshnessPolicy, LabelState, LocalUrlMap,
    PartitionState, ProofDocumentManifest, QuiverManifest, QuiverSummary, RegistryIndexReport,
    RegistryIndexRow, RollupPreview, SourceKind,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "fletch")]
#[command(about = "Fetch/cache manifests for reproducible data pipelines")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
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

#[derive(Debug, Subcommand)]
enum PartitionCommands {
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
enum CacheCommands {
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
enum QuiverCommands {
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
enum GraphCommands {
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
enum RegistryCommands {
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
enum TipCommands {
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
enum PublishCommands {
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
    /// Generate a CROP-indexable report from a cache manifest.
    CropIndex {
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
    /// Generate PROOF document anchors from a CROP index report.
    ProofDocs {
        /// Path to a fletch.crop-index.v1 JSON file.
        #[arg(long)]
        crop_index: PathBuf,
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
    /// Generate stable local URLs from a PROOF document manifest.
    LocalUrlMap {
        /// Path to a fletch.proof-docs.v1 JSON file.
        #[arg(long)]
        proof_docs: PathBuf,
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
    /// Summarize publisher inputs for downstream CROP/PROOF backends.
    Bundle {
        /// Path to a fletch.crop-index.v1 JSON file.
        #[arg(long)]
        crop_index: PathBuf,
        /// Path to a fletch.proof-docs.v1 JSON file.
        #[arg(long)]
        proof_docs: PathBuf,
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
enum MergeCommands {
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

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliSourceKind {
    Http,
    File,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliFreshness {
    Immutable,
    MaxAgeDays,
    AlwaysCheck,
}

impl From<CliSourceKind> for SourceKind {
    fn from(value: CliSourceKind) -> Self {
        match value {
            CliSourceKind::Http => Self::Http,
            CliSourceKind::File => Self::File,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Plan {
            dataset_id,
            url,
            source_kind,
            headers,
            freshness,
            max_age_days,
            output,
        } => {
            let mut plan = fetch_plan_with_kind(dataset_id, url, source_kind.into())?;
            plan.source.headers = parse_headers(headers)?;
            plan.cache_policy.freshness = freshness_policy(freshness, max_age_days)?;
            let json = serde_json::to_string_pretty(&plan)?;
            if let Some(output) = output {
                fs::write(output, json)?;
            } else {
                println!("{json}");
            }
        }
        Commands::Key {
            dataset_id,
            url,
            headers,
        } => {
            let mut plan = fetch_plan(dataset_id, url)?;
            plan.source.headers = parse_headers(headers)?;
            println!("{}", cache_key(&plan));
        }
        Commands::Fetch {
            dataset_id,
            url,
            source_kind,
            headers,
            cache_root,
            expect_sha256,
            trusted_manifest,
            max_bytes_per_second,
            timeout_ms,
            retry_attempts,
            force,
            offline,
            freshness,
            max_age_days,
            output,
        } => {
            let mut plan = fetch_plan_with_kind(dataset_id, url, source_kind.into())?;
            plan.source.headers = parse_headers(headers)?;
            plan.cache_policy.freshness = freshness_policy(freshness, max_age_days)?;
            let mut options = FetchOptions::new(&cache_root)
                .with_force(force)
                .with_offline(offline);
            if let Some(expected) = expect_sha256 {
                options = options.with_expected_sha256(expected);
            }
            if let Some(trusted_manifest) = trusted_manifest {
                let trusted_manifest = read_manifest(&trusted_manifest)?;
                options = options.with_trusted_manifest(&trusted_manifest);
            }
            if let Some(max_bytes_per_second) = max_bytes_per_second {
                options = options.with_max_bytes_per_second(max_bytes_per_second);
            }
            if let Some(timeout_ms) = timeout_ms {
                options = options.with_timeout_ms(timeout_ms);
            }
            options = options.with_retry_attempts(retry_attempts);
            let outcome = fetch_to_cache(&plan, options)?;
            write_fetch_manifest(&cache_root, outcome.entry, output)?;
        }
        Commands::FetchPlan {
            plan,
            cache_root,
            expect_sha256,
            trusted_manifest,
            max_bytes_per_second,
            timeout_ms,
            retry_attempts,
            force,
            offline,
            output,
        } => {
            let plan = read_plan(&plan)?;
            let mut options = FetchOptions::new(&cache_root)
                .with_force(force)
                .with_offline(offline);
            if let Some(expected) = expect_sha256 {
                options = options.with_expected_sha256(expected);
            }
            if let Some(trusted_manifest) = trusted_manifest {
                let trusted_manifest = read_manifest(&trusted_manifest)?;
                options = options.with_trusted_manifest(&trusted_manifest);
            }
            if let Some(max_bytes_per_second) = max_bytes_per_second {
                options = options.with_max_bytes_per_second(max_bytes_per_second);
            }
            if let Some(timeout_ms) = timeout_ms {
                options = options.with_timeout_ms(timeout_ms);
            }
            options = options.with_retry_attempts(retry_attempts);
            let outcome = fetch_to_cache(&plan, options)?;
            write_fetch_manifest(&cache_root, outcome.entry, output)?;
        }
        Commands::Cache { command } => match command {
            CacheCommands::Index {
                manifest,
                dataset_id,
                cache_key,
                verified,
                offset,
                limit,
                output,
            } => {
                let manifest = read_manifest(&manifest)?;
                let index = cache_index_from_manifest(&manifest);
                write_json(
                    &slice_cache_index_report(
                        &index,
                        dataset_id.as_deref(),
                        cache_key.as_deref(),
                        verified,
                        offset,
                        limit,
                    ),
                    output,
                )?;
            }
            CacheCommands::IndexGate {
                manifest,
                expected_registries,
                expected_dataset_ids,
                require_all_expected,
                allow_unverified,
                gate,
                output,
            } => {
                let manifest = read_manifest(&manifest)?;
                let index = cache_index_from_manifest(&manifest);
                let expected_dataset_ids =
                    expected_dataset_ids_from_inputs(expected_dataset_ids, expected_registries)?;
                let report = cache_index_gate_report(
                    &index,
                    &CacheIndexGatePolicy {
                        expected_dataset_ids,
                        require_verified: !allow_unverified,
                        allow_missing_expected: !require_all_expected,
                    },
                );
                let passed = report.passed;
                write_json(&report, output)?;
                if gate && !passed {
                    bail!("cache index gate failed");
                }
            }
            CacheCommands::IndexDiff {
                base_index,
                candidate_index,
                output,
            } => {
                let base_index = read_cache_index(&base_index)?;
                let candidate_index = read_cache_index(&candidate_index)?;
                write_json(&cache_index_diff(&base_index, &candidate_index), output)?;
            }
            CacheCommands::List { manifest, output } => {
                let manifest = read_manifest(&manifest)?;
                write_json(cache_list(&manifest), output)?;
            }
            CacheCommands::Verify { manifest, output } => {
                let manifest = read_manifest(&manifest)?;
                write_json(&verify_cache_manifest(&manifest)?, output)?;
            }
            CacheCommands::Status {
                manifest,
                freshness,
                max_age_days,
                output,
            } => {
                let manifest = read_manifest(&manifest)?;
                let freshness = freshness_policy(freshness, max_age_days)?;
                write_json(&inspect_cache_manifest(&manifest, &freshness)?, output)?;
            }
            CacheCommands::Summary {
                manifest,
                freshness,
                max_age_days,
                output,
            } => {
                let manifest = read_manifest(&manifest)?;
                let freshness = freshness_policy(freshness, max_age_days)?;
                write_json(&summarize_cache_manifest(&manifest, &freshness)?, output)?;
            }
            CacheCommands::OfflineReport {
                manifest,
                freshness,
                max_age_days,
                output,
            } => {
                let manifest = read_manifest(&manifest)?;
                let freshness = freshness_policy(freshness, max_age_days)?;
                write_json(&offline_cache_report(&manifest, &freshness)?, output)?;
            }
            CacheCommands::Prune { manifest, output } => {
                let manifest = read_manifest(&manifest)?;
                write_json(&plan_cache_prune(&manifest)?, output)?;
            }
        },
        Commands::Quiver { command } => match command {
            QuiverCommands::Export {
                manifest,
                quiver_id,
                output_dir,
                output,
            } => {
                let manifest = read_manifest(&manifest)?;
                let exported = export_quiver(&manifest, quiver_id, output_dir)?;
                write_json(&exported.manifest, output)?;
            }
            QuiverCommands::Import {
                quiver_dir,
                cache_root,
                output,
            } => {
                let imported = import_quiver(quiver_dir, cache_root)?;
                write_json(&imported.staged_manifest, output)?;
            }
            QuiverCommands::Summary { quiver, output } => {
                let quiver = read_quiver_manifest(&quiver)?;
                write_json(&summarize_quiver(&quiver), output)?;
            }
            QuiverCommands::Verify { quiver_dir, output } => {
                write_json(&verify_quiver_bundle(quiver_dir)?, output)?;
            }
            QuiverCommands::Graph { quiver, output } => {
                let quiver = read_quiver_manifest(&quiver)?;
                write_json(&graph_from_quiver(&quiver), output)?;
            }
            QuiverCommands::MergeReady {
                quiver,
                alias_id,
                status,
                offset,
                limit,
                output,
            } => {
                let quiver = read_quiver_manifest(&quiver)?;
                let report = quiver_merge_ready_report(&quiver, alias_id);
                write_json(
                    &slice_quiver_merge_ready_report(&report, status.as_deref(), offset, limit),
                    output,
                )?;
            }
        },
        Commands::Graph { command } => match command {
            GraphCommands::Export { manifest, output } => {
                let manifest = read_manifest(&manifest)?;
                write_json(&graph_from_manifest(&manifest), output)?;
            }
        },
        Commands::Registry { command } => match command {
            RegistryCommands::Index {
                files,
                follow,
                output,
            } => {
                let registries = read_registry_inputs(&files, follow)?;
                write_json(&registry_index_from_registries(&registries), output)?;
            }
            RegistryCommands::Search {
                index,
                tags,
                metadata,
                text,
                offset,
                limit,
                output,
            } => {
                let index = read_registry_index(&index)?;
                let metadata_filters = parse_key_value_filters(metadata)?;
                write_json(
                    &search_registry_index(
                        &index,
                        &tags,
                        &metadata_filters,
                        text.as_deref(),
                        offset,
                        limit,
                    ),
                    output,
                )?;
            }
            RegistryCommands::Web {
                index,
                files,
                follow,
                host,
                port,
                open,
            } => {
                let index = read_registry_web_index(index, files, follow)?;
                serve_registry_web(index, host, port, open)?;
            }
            RegistryCommands::Graph { file, output } => {
                let registry = read_registry(&file)?;
                write_json(&graph_from_registry(&registry), output)?;
            }
            RegistryCommands::Flight {
                file,
                fletch_ids,
                output,
            } => {
                let registry = read_registry(&file)?;
                write_json(&dry_run_flight(&registry, &fletch_ids), output)?;
            }
            RegistryCommands::AdapterSources {
                file,
                adapter_owned,
                offset,
                limit,
                output,
            } => {
                let registry = read_registry(&file)?;
                let report = adapter_sources_from_registry(&registry);
                write_json(
                    &slice_adapter_source_report(&report, adapter_owned, offset, limit),
                    output,
                )?;
            }
            RegistryCommands::Validate {
                file,
                severity,
                offset,
                limit,
                output,
            } => {
                let registry = read_registry(&file)?;
                let report = validate_registry(&registry);
                write_json(
                    &slice_registry_validation_report(&report, severity.as_deref(), offset, limit),
                    output,
                )?;
            }
            RegistryCommands::ArchivePreview {
                file,
                archive_fletch_id,
                offset,
                limit,
                output,
            } => {
                let registry = read_registry(&file)?;
                let preview = preview_archive_expansion(&registry, archive_fletch_id);
                write_json(
                    &slice_archive_expansion_preview(&preview, offset, limit),
                    output,
                )?;
            }
            RegistryCommands::Handoff {
                file,
                fletch_ids,
                output,
            } => {
                let registry = read_registry(&file)?;
                write_json(&adapter_handoff_report(&registry, &fletch_ids), output)?;
            }
        },
        Commands::Tip { command } => match command {
            TipCommands::FromManifest {
                manifest,
                max_bytes,
                output,
            } => {
                let manifest = read_manifest(&manifest)?;
                write_json(&tips_from_manifest(&manifest, max_bytes)?, output)?;
            }
        },
        Commands::Publish { command } => match command {
            PublishCommands::FromManifest {
                manifest,
                freshness,
                max_age_days,
                max_tip_bytes,
                output,
            } => {
                let manifest = read_manifest(&manifest)?;
                let freshness = freshness_policy(freshness, max_age_days)?;
                write_json(
                    &publish_report_from_manifest(&manifest, &freshness, max_tip_bytes)?,
                    output,
                )?;
            }
            PublishCommands::CropIndex {
                manifest,
                freshness,
                max_age_days,
                max_tip_bytes,
                row_type,
                offset,
                limit,
                output,
            } => {
                let manifest = read_manifest(&manifest)?;
                let freshness = freshness_policy(freshness, max_age_days)?;
                let index = crop_index_from_manifest(&manifest, &freshness, max_tip_bytes)?;
                write_json(
                    &slice_crop_index_report(&index, row_type.as_deref(), offset, limit),
                    output,
                )?;
            }
            PublishCommands::ProofDocs {
                crop_index,
                offset,
                limit,
                output,
            } => {
                let crop_index = read_crop_index(&crop_index)?;
                let docs = proof_document_manifest(&crop_index);
                write_json(&slice_proof_document_manifest(&docs, offset, limit), output)?;
            }
            PublishCommands::LocalUrlMap {
                proof_docs,
                base_path,
                offset,
                limit,
                output,
            } => {
                let proof_docs = read_proof_docs(&proof_docs)?;
                let urls = local_url_map(&proof_docs, base_path);
                write_json(&slice_local_url_map(&urls, offset, limit), output)?;
            }
            PublishCommands::Bundle {
                crop_index,
                proof_docs,
                local_url_map,
                quiver_summary,
                adapter_handoff,
                output,
            } => {
                let crop_index = read_crop_index(&crop_index)?;
                let proof_docs = read_proof_docs(&proof_docs)?;
                let local_url_map = read_local_url_map(&local_url_map)?;
                let quiver_summary = quiver_summary
                    .as_ref()
                    .map(read_quiver_summary)
                    .transpose()?;
                let adapter_handoff = adapter_handoff
                    .as_ref()
                    .map(read_adapter_handoff)
                    .transpose()?;
                write_json(
                    &publisher_bundle_report(
                        &crop_index,
                        &proof_docs,
                        &local_url_map,
                        quiver_summary.as_ref(),
                        adapter_handoff.as_ref(),
                    ),
                    output,
                )?;
            }
        },
        Commands::Merge { command } => match command {
            MergeCommands::Preview {
                active,
                candidate,
                output,
            } => {
                let active = read_manifest(&active)?;
                let candidate = read_manifest(&candidate)?;
                write_json(&preview_manifest_merge(&active, &candidate), output)?;
            }
            MergeCommands::AliasState {
                manifest,
                alias_id,
                dataset_id,
                output,
            } => {
                let manifest = read_manifest(&manifest)?;
                write_json(
                    &alias_state_from_manifest(&manifest, alias_id, dataset_id)?,
                    output,
                )?;
            }
            MergeCommands::LabelState {
                alias_state,
                label_id,
                pin,
                output,
            } => {
                let alias_state = read_alias_state(&alias_state)?;
                write_json(
                    &label_state_from_aliases(&alias_state, label_id, pin),
                    output,
                )?;
            }
            MergeCommands::RollbackPreview {
                alias_state,
                label_state,
                output,
            } => {
                let alias_state = read_alias_state(&alias_state)?;
                let label_state = read_label_state(&label_state)?;
                write_json(&preview_rollback(&alias_state, &label_state), output)?;
            }
        },
        Commands::Partition { command } => match command {
            PartitionCommands::State {
                manifest,
                group_id,
                offset,
                limit,
                output,
            } => {
                let manifest = read_manifest(&manifest)?;
                let state = partition_state_from_manifest(&manifest, group_id);
                write_json(&slice_partition_state(&state, offset, limit), output)?;
            }
            PartitionCommands::RollupPreview {
                partition_state,
                rollup_id,
                child_partition_ids,
                output,
            } => {
                let partition_state = read_partition_state(&partition_state)?;
                write_json(
                    &preview_rollup_edges(&partition_state, rollup_id, &child_partition_ids),
                    output,
                )?;
            }
            PartitionCommands::InvalidationReport {
                partition_state,
                stale_partition_ids,
                folded_partition_ids,
                superseded_partition_ids,
                output,
            } => {
                let partition_state = read_partition_state(&partition_state)?;
                write_json(
                    &partition_invalidation_report(
                        &partition_state,
                        &stale_partition_ids,
                        &folded_partition_ids,
                        &superseded_partition_ids,
                    ),
                    output,
                )?;
            }
            PartitionCommands::ActiveSet {
                partition_state,
                alias_state,
                label_state,
                rollup_preview,
                active,
                offset,
                limit,
                output,
            } => {
                let partition_state = read_partition_state(&partition_state)?;
                let alias_state = alias_state.as_ref().map(read_alias_state).transpose()?;
                let label_state = label_state.as_ref().map(read_label_state).transpose()?;
                let rollup_preview = rollup_preview
                    .as_ref()
                    .map(read_rollup_preview)
                    .transpose()?;
                let active_set = active_partition_set(
                    &partition_state,
                    alias_state.as_ref(),
                    label_state.as_ref(),
                    rollup_preview.as_ref(),
                );
                write_json(
                    &slice_active_partition_set(&active_set, active, offset, limit),
                    output,
                )?;
            }
        },
    }
    Ok(())
}

fn freshness_policy(freshness: CliFreshness, max_age_days: Option<u32>) -> Result<FreshnessPolicy> {
    match freshness {
        CliFreshness::Immutable => Ok(FreshnessPolicy::Immutable),
        CliFreshness::AlwaysCheck => Ok(FreshnessPolicy::AlwaysCheck),
        CliFreshness::MaxAgeDays => Ok(FreshnessPolicy::MaxAgeDays(
            max_age_days.ok_or_else(|| anyhow::anyhow!("--max-age-days is required"))?,
        )),
    }
}

fn parse_headers(headers: Vec<String>) -> Result<BTreeMap<String, String>> {
    let mut parsed = BTreeMap::new();
    for header in headers {
        let Some((name, value)) = header.split_once('=') else {
            bail!("header must be formatted as name=value: {header}");
        };
        let name = name.trim();
        if name.is_empty() {
            bail!("header name must not be empty: {header}");
        }
        parsed.insert(name.to_string(), value.to_string());
    }
    Ok(parsed)
}

fn parse_key_value_filters(filters: Vec<String>) -> Result<Vec<(String, String)>> {
    let mut parsed = Vec::new();
    for filter in filters {
        parsed.push(parse_key_value_filter(&filter)?);
    }
    Ok(parsed)
}

fn parse_key_value_filter(filter: &str) -> Result<(String, String)> {
    let Some((name, value)) = filter.split_once('=') else {
        bail!("filter must be formatted as name=value: {filter}");
    };
    let name = name.trim();
    if name.is_empty() {
        bail!("filter name must not be empty: {filter}");
    }
    Ok((name.to_string(), value.to_string()))
}

fn read_manifest(path: &PathBuf) -> Result<CacheManifest> {
    Ok(read_cache_manifest_json(path)?)
}

fn read_plan(path: &PathBuf) -> Result<FetchPlan> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn read_registry(path: &PathBuf) -> Result<FletchRegistry> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn read_registry_inputs(files: &[PathBuf], follow: bool) -> Result<Vec<FletchRegistry>> {
    let mut registries = files
        .iter()
        .map(read_registry)
        .collect::<Result<Vec<_>>>()?;
    if follow {
        let followed = follow_registry_pointers(&registries)?;
        registries.extend(followed);
    }
    Ok(registries)
}

fn follow_registry_pointers(registries: &[FletchRegistry]) -> Result<Vec<FletchRegistry>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(format!("fletch-cli/{}", env!("CARGO_PKG_VERSION")))
        .build()?;
    let mut followed_urls = BTreeSet::new();
    let mut followed = Vec::new();
    for registry in registries {
        for definition in &registry.fletches {
            if !definition.tags.iter().any(|tag| tag == "repo-registry") {
                continue;
            }
            for shaft in &definition.shafts {
                match shaft.kind {
                    SourceKind::Http => {
                        let urls = registry_urls_from_pointer(&client, &shaft.url)
                            .with_context(|| format!("failed to resolve {}", shaft.url))?;
                        for url in urls {
                            if followed_urls.insert(url.clone()) {
                                followed.push(fetch_registry_url(&client, &url)?);
                            }
                        }
                    }
                    SourceKind::File => {
                        if followed_urls.insert(shaft.url.clone()) {
                            followed.push(read_registry(&PathBuf::from(&shaft.url))?);
                        }
                    }
                    SourceKind::Adapter => {}
                }
            }
        }
    }
    Ok(followed)
}

fn registry_urls_from_pointer(
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<Vec<String>> {
    if let Some(contents_url) = github_tree_url_to_contents_api(url) {
        return registry_urls_from_github_contents(client, &contents_url);
    }
    if let Some(raw_url) = github_blob_url_to_raw(url) {
        return Ok(vec![raw_url]);
    }
    if url.contains("api.github.com/repos/") && url.contains("/contents/") {
        return registry_urls_from_github_contents(client, url);
    }
    Ok(vec![url.to_string()])
}

fn registry_urls_from_github_contents(
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<Vec<String>> {
    let text = client
        .get(url)
        .send()?
        .error_for_status()?
        .text()
        .with_context(|| format!("failed to read GitHub contents response from {url}"))?;
    let value: serde_json::Value = serde_json::from_str(&text)
        .with_context(|| format!("failed to parse GitHub contents response from {url}"))?;
    let mut urls = Vec::new();
    match value {
        serde_json::Value::Array(items) => {
            for item in items {
                if let Some(download_url) = github_contents_download_url(&item) {
                    urls.push(download_url);
                }
            }
        }
        serde_json::Value::Object(_) => {
            if let Some(download_url) = github_contents_download_url(&value) {
                urls.push(download_url);
            }
        }
        _ => bail!("GitHub contents response from {url} was not an object or array"),
    }
    if urls.is_empty() {
        bail!("GitHub contents response from {url} did not contain registry JSON download URLs");
    }
    Ok(urls)
}

fn github_contents_download_url(value: &serde_json::Value) -> Option<String> {
    let name = value.get("name")?.as_str()?;
    if !name.ends_with(".json") {
        return None;
    }
    value
        .get("download_url")
        .and_then(|download_url| download_url.as_str())
        .map(ToString::to_string)
}

fn fetch_registry_url(client: &reqwest::blocking::Client, url: &str) -> Result<FletchRegistry> {
    let text = client
        .get(url)
        .send()?
        .error_for_status()?
        .text()
        .with_context(|| format!("failed to read registry JSON from {url}"))?;
    let mut registry: FletchRegistry = serde_json::from_str(&text)
        .with_context(|| format!("failed to parse registry JSON from {url}"))?;
    annotate_registry_source(&mut registry, url);
    Ok(registry)
}

fn annotate_registry_source(registry: &mut FletchRegistry, url: &str) {
    let base_url = raw_github_repo_base_url(url);
    for definition in &mut registry.fletches {
        definition
            .metadata
            .entry("registry_source_url".to_string())
            .or_insert_with(|| url.to_string());
        if let Some(base_url) = &base_url {
            definition
                .metadata
                .entry("registry_source_base_url".to_string())
                .or_insert_with(|| base_url.clone());
        }
    }
}

fn raw_github_repo_base_url(url: &str) -> Option<String> {
    let path = url.strip_prefix("https://raw.githubusercontent.com/")?;
    let parts = path.split('/').collect::<Vec<_>>();
    if parts.len() < 4 {
        return None;
    }
    Some(format!(
        "https://raw.githubusercontent.com/{}/{}/{}/",
        parts[0], parts[1], parts[2]
    ))
}

fn github_tree_url_to_contents_api(url: &str) -> Option<String> {
    github_url_to_contents_api(url, "tree")
}

fn github_blob_url_to_raw(url: &str) -> Option<String> {
    let (owner, repo, branch, path) = github_url_parts(url, "blob")?;
    Some(format!(
        "https://raw.githubusercontent.com/{owner}/{repo}/{branch}/{path}"
    ))
}

fn github_url_to_contents_api(url: &str, marker: &str) -> Option<String> {
    let (owner, repo, branch, path) = github_url_parts(url, marker)?;
    Some(format!(
        "https://api.github.com/repos/{owner}/{repo}/contents/{path}?ref={branch}"
    ))
}

fn github_url_parts(url: &str, marker: &str) -> Option<(String, String, String, String)> {
    let path = url.strip_prefix("https://github.com/")?;
    let parts = path.split('/').collect::<Vec<_>>();
    if parts.len() < 5 || parts[2] != marker {
        return None;
    }
    let rest = parts[4..].join("/");
    Some((
        parts[0].to_string(),
        parts[1].to_string(),
        parts[3].to_string(),
        rest,
    ))
}

fn read_registry_index(path: &PathBuf) -> Result<RegistryIndexReport> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn read_registry_web_index(
    index: Option<PathBuf>,
    files: Vec<PathBuf>,
    follow: bool,
) -> Result<RegistryIndexReport> {
    match (index, files.is_empty()) {
        (Some(index), true) => read_registry_index(&index),
        (None, false) => {
            let registries = read_registry_inputs(&files, follow)?;
            Ok(registry_index_from_registries(&registries))
        }
        (Some(_), false) => bail!("use either --index or --file inputs for registry web, not both"),
        (None, true) => bail!("registry web requires --index or at least one --file input"),
    }
}

fn serve_registry_web(
    index: RegistryIndexReport,
    host: String,
    port: u16,
    open: bool,
) -> Result<()> {
    let listener = TcpListener::bind((host.as_str(), port))?;
    let address = listener.local_addr()?;
    let url = format!("http://{address}/");
    eprintln!("FLETCH registry web listening at {url}");
    if open {
        open_browser(&url)?;
    }
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(error) = handle_registry_web_request(stream, &index) {
                    eprintln!("FLETCH registry web request failed: {error:#}");
                }
            }
            Err(error) => eprintln!("FLETCH registry web connection failed: {error}"),
        }
    }
    Ok(())
}

fn open_browser(url: &str) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        let mut command = std::process::Command::new("cmd");
        command.args(["/C", "start", "", url]);
        return run_browser_launcher(command);
    }

    #[cfg(target_os = "macos")]
    {
        let mut command = std::process::Command::new("open");
        command.arg(url);
        return run_browser_launcher(command);
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        let mut command = std::process::Command::new("xdg-open");
        command.arg(url);
        return run_browser_launcher(command);
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", unix)))]
    bail!("--open is not supported on this platform; open {url} manually");
}

fn run_browser_launcher(mut command: std::process::Command) -> Result<()> {
    let status = command.status().context("failed to launch browser")?;
    if !status.success() {
        bail!("browser launcher exited with status {status}");
    }
    Ok(())
}

fn handle_registry_web_request(mut stream: TcpStream, index: &RegistryIndexReport) -> Result<()> {
    let mut buffer = [0; 8192];
    let bytes_read = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let Some(request_line) = request.lines().next() else {
        write_http_response(
            &mut stream,
            "400 Bad Request",
            "text/plain",
            "missing request",
        )?;
        return Ok(());
    };
    let parts = request_line.split_whitespace().collect::<Vec<_>>();
    if parts.len() < 2 || parts[0] != "GET" {
        write_http_response(
            &mut stream,
            "405 Method Not Allowed",
            "text/plain",
            "only GET is supported",
        )?;
        return Ok(());
    }
    let (path, query) = split_path_query(parts[1]);
    match path {
        "/" | "/index.html" => write_http_response(
            &mut stream,
            "200 OK",
            "text/html; charset=utf-8",
            REGISTRY_WEB_HTML,
        ),
        "/api/summary" => write_json_response(
            &mut stream,
            &serde_json::json!({
                "schema_version": index.schema_version,
                "registry_count": index.registry_count,
                "fletch_count": index.fletch_count,
                "row_count": index.row_count
            }),
        ),
        "/api/facets" => write_json_response(&mut stream, &registry_web_facets(index)),
        "/api/search" => {
            let query = parse_query(query);
            let tags = query.get("tag").cloned().unwrap_or_default();
            let metadata = query
                .get("metadata")
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .map(|filter| parse_key_value_filter(&filter))
                .collect::<Result<Vec<_>>>()?;
            let text = query.get("text").and_then(|values| values.first()).cloned();
            let offset = query
                .get("offset")
                .and_then(|values| values.first())
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(0);
            let limit = query
                .get("limit")
                .and_then(|values| values.first())
                .and_then(|value| value.parse::<usize>().ok())
                .or(Some(50));
            let sort = query.get("sort").and_then(|values| values.first());
            let direction = query.get("direction").and_then(|values| values.first());
            let report = search_registry_web_index(
                index,
                &tags,
                &metadata,
                text.as_deref(),
                offset,
                limit,
                sort.map(String::as_str),
                direction.map(String::as_str),
            );
            write_json_response(&mut stream, &report)
        }
        "/api/row" => {
            let query = parse_query(query);
            let registry_id = query.get("registry_id").and_then(|values| values.first());
            let fletch_id = query.get("fletch_id").and_then(|values| values.first());
            let row = registry_id
                .zip(fletch_id)
                .and_then(|(registry_id, fletch_id)| {
                    find_registry_index_row(index, registry_id, fletch_id)
                });
            if let Some(row) = row {
                write_json_response(&mut stream, row)
            } else {
                write_http_response(&mut stream, "404 Not Found", "text/plain", "row not found")
            }
        }
        "/api/source" => {
            let query = parse_query(query);
            let registry_id = query.get("registry_id").and_then(|values| values.first());
            let fletch_id = query.get("fletch_id").and_then(|values| values.first());
            let source_index = query
                .get("source")
                .and_then(|values| values.first())
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(0);
            let line_start = query
                .get("line_start")
                .and_then(|values| values.first())
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(1);
            let line_count = query
                .get("line_count")
                .and_then(|values| values.first())
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(80);
            let row = registry_id
                .zip(fletch_id)
                .and_then(|(registry_id, fletch_id)| {
                    find_registry_index_row(index, registry_id, fletch_id)
                });
            if let Some(row) = row {
                write_json_response(
                    &mut stream,
                    &load_registry_source_preview(row, source_index, line_start, line_count)?,
                )
            } else {
                write_http_response(&mut stream, "404 Not Found", "text/plain", "row not found")
            }
        }
        _ => write_http_response(&mut stream, "404 Not Found", "text/plain", "not found"),
    }
}

fn split_path_query(target: &str) -> (&str, &str) {
    target.split_once('?').unwrap_or((target, ""))
}

fn parse_query(query: &str) -> BTreeMap<String, Vec<String>> {
    let mut parsed = BTreeMap::new();
    for part in query.split('&').filter(|part| !part.is_empty()) {
        let (key, value) = part.split_once('=').unwrap_or((part, ""));
        parsed
            .entry(url_decode(key))
            .or_insert_with(Vec::new)
            .push(url_decode(value));
    }
    parsed
}

fn url_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'+' => {
                decoded.push(b' ');
                index += 1;
            }
            b'%' if index + 2 < bytes.len() => {
                let hex = &value[index + 1..index + 3];
                if let Ok(byte) = u8::from_str_radix(hex, 16) {
                    decoded.push(byte);
                    index += 3;
                } else {
                    decoded.push(bytes[index]);
                    index += 1;
                }
            }
            byte => {
                decoded.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8_lossy(&decoded).into_owned()
}

fn find_registry_index_row<'a>(
    index: &'a RegistryIndexReport,
    registry_id: &str,
    fletch_id: &str,
) -> Option<&'a RegistryIndexRow> {
    index
        .rows
        .iter()
        .find(|row| row.registry_id == registry_id && row.fletch_id == fletch_id)
}

fn registry_web_facets(index: &RegistryIndexReport) -> serde_json::Value {
    let mut registries = BTreeMap::new();
    let mut tags = BTreeMap::new();
    let mut metadata = BTreeMap::from([
        ("owner_repo".to_string(), BTreeMap::new()),
        ("domain".to_string(), BTreeMap::new()),
        ("asset_kind".to_string(), BTreeMap::new()),
        ("fetch_policy".to_string(), BTreeMap::new()),
    ]);
    for row in &index.rows {
        increment_facet(&mut registries, &row.registry_id);
        for tag in &row.tags {
            increment_facet(&mut tags, tag);
        }
        for (key, values) in &mut metadata {
            if let Some(value) = row.metadata.get(key) {
                increment_facet(values, value);
            }
        }
    }
    serde_json::json!({
        "registries": top_facets(registries, 40),
        "tags": top_facets(tags, 80),
        "metadata": metadata
            .into_iter()
            .map(|(key, values)| (key, top_facets(values, 40)))
            .collect::<BTreeMap<_, _>>()
    })
}

fn increment_facet(counts: &mut BTreeMap<String, usize>, value: &str) {
    *counts.entry(value.to_string()).or_insert(0) += 1;
}

fn top_facets(counts: BTreeMap<String, usize>, limit: usize) -> Vec<serde_json::Value> {
    let mut values = counts.into_iter().collect::<Vec<_>>();
    values.sort_by(|(left_value, left_count), (right_value, right_count)| {
        right_count
            .cmp(left_count)
            .then_with(|| left_value.cmp(right_value))
    });
    values
        .into_iter()
        .take(limit)
        .map(|(value, count)| serde_json::json!({ "value": value, "count": count }))
        .collect()
}

fn search_registry_web_index(
    index: &RegistryIndexReport,
    tags: &[String],
    metadata_filters: &[(String, String)],
    text: Option<&str>,
    offset: usize,
    limit: Option<usize>,
    sort: Option<&str>,
    direction: Option<&str>,
) -> fletch_core::RegistrySearchReport {
    let mut report = search_registry_index(index, tags, metadata_filters, text, 0, None);
    let sort = sort.unwrap_or("index");
    let descending = direction == Some("desc");
    sort_registry_web_rows(&mut report.rows, sort, descending);
    if sort != "index" {
        report.query.insert("sort".to_string(), sort.to_string());
    }
    if descending {
        report
            .query
            .insert("direction".to_string(), "desc".to_string());
    }
    report.rows = report
        .rows
        .iter()
        .skip(offset)
        .take(limit.unwrap_or(usize::MAX))
        .cloned()
        .collect();
    report
}

fn sort_registry_web_rows(rows: &mut [RegistryIndexRow], sort: &str, descending: bool) {
    match sort {
        "fletch_id" => rows.sort_by(|left, right| left.fletch_id.cmp(&right.fletch_id)),
        "registry_id" => rows.sort_by(|left, right| left.registry_id.cmp(&right.registry_id)),
        "owner_repo" => {
            rows.sort_by(|left, right| metadata_sort_order(left, right, "owner_repo", descending))
        }
        "domain" => {
            rows.sort_by(|left, right| metadata_sort_order(left, right, "domain", descending))
        }
        "asset_kind" => {
            rows.sort_by(|left, right| metadata_sort_order(left, right, "asset_kind", descending))
        }
        "node_kind" => rows.sort_by(|left, right| {
            format!("{:?}", left.node_kind)
                .cmp(&format!("{:?}", right.node_kind))
                .then_with(|| left.fletch_id.cmp(&right.fletch_id))
        }),
        _ => {}
    }
    if descending && !matches!(sort, "owner_repo" | "domain" | "asset_kind") {
        rows.reverse();
    }
}

fn metadata_sort_order(
    left: &RegistryIndexRow,
    right: &RegistryIndexRow,
    key: &str,
    descending: bool,
) -> std::cmp::Ordering {
    match (left.metadata.get(key), right.metadata.get(key)) {
        (Some(left_value), Some(right_value)) if descending => right_value
            .cmp(left_value)
            .then_with(|| left.fletch_id.cmp(&right.fletch_id)),
        (Some(left_value), Some(right_value)) => left_value
            .cmp(right_value)
            .then_with(|| left.fletch_id.cmp(&right.fletch_id)),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => left.fletch_id.cmp(&right.fletch_id),
    }
}

fn load_registry_source_preview(
    row: &RegistryIndexRow,
    source_index: usize,
    line_start: usize,
    line_count: usize,
) -> Result<serde_json::Value> {
    let Some(source_url) = row.source_urls.get(source_index) else {
        bail!("source index {source_index} is out of range");
    };
    let resolved_url = resolve_registry_source_url(row, source_url);
    let bytes = if resolved_url.starts_with("http://") || resolved_url.starts_with("https://") {
        reqwest::blocking::Client::builder()
            .user_agent(format!("fletch-cli/{}", env!("CARGO_PKG_VERSION")))
            .build()?
            .get(&resolved_url)
            .send()?
            .error_for_status()?
            .bytes()?
            .to_vec()
    } else {
        fs::read(&resolved_url)?
    };
    let limit = 65_536;
    let truncated = bytes.len() > limit;
    let preview_bytes = &bytes[..bytes.len().min(limit)];
    let text = String::from_utf8_lossy(preview_bytes).into_owned();
    let lines = text.lines().collect::<Vec<_>>();
    let total_line_count = lines.len();
    let start = line_start.max(1);
    let selected_lines = lines
        .iter()
        .enumerate()
        .skip(start.saturating_sub(1))
        .take(line_count.min(500))
        .map(|(index, line)| serde_json::json!({ "number": index + 1, "text": line }))
        .collect::<Vec<_>>();
    Ok(serde_json::json!({
        "registry_id": row.registry_id,
        "fletch_id": row.fletch_id,
        "source_index": source_index,
        "source_url": source_url,
        "resolved_url": resolved_url,
        "byte_count": bytes.len(),
        "preview_byte_count": preview_bytes.len(),
        "truncated": truncated,
        "total_line_count": total_line_count,
        "line_start": start,
        "line_count": selected_lines.len(),
        "lines": selected_lines,
        "json_outline": json_outline(&text),
        "text": text
    }))
}

fn json_outline(text: &str) -> Option<serde_json::Value> {
    let value = serde_json::from_str::<serde_json::Value>(text).ok()?;
    Some(match value {
        serde_json::Value::Object(object) => serde_json::json!({
            "kind": "object",
            "keys": object.keys().take(80).cloned().collect::<Vec<_>>(),
            "key_count": object.len()
        }),
        serde_json::Value::Array(array) => serde_json::json!({
            "kind": "array",
            "length": array.len()
        }),
        serde_json::Value::String(_) => serde_json::json!({ "kind": "string" }),
        serde_json::Value::Number(_) => serde_json::json!({ "kind": "number" }),
        serde_json::Value::Bool(_) => serde_json::json!({ "kind": "bool" }),
        serde_json::Value::Null => serde_json::json!({ "kind": "null" }),
    })
}

fn resolve_registry_source_url(row: &RegistryIndexRow, source_url: &str) -> String {
    if source_url.starts_with("http://") || source_url.starts_with("https://") {
        return source_url.to_string();
    }
    if let Some(base_url) = row.metadata.get("registry_source_base_url") {
        return format!(
            "{}{}",
            base_url.trim_end_matches('/'),
            format!("/{source_url}")
        );
    }
    source_url.to_string()
}

fn write_json_response<T: serde::Serialize + ?Sized>(
    stream: &mut TcpStream,
    value: &T,
) -> Result<()> {
    let body = serde_json::to_string_pretty(value)?;
    write_http_response(stream, "200 OK", "application/json; charset=utf-8", &body)
}

fn write_http_response(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &str,
) -> Result<()> {
    write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    )?;
    Ok(())
}

const REGISTRY_WEB_HTML: &str = r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>FLETCH Registry Search</title>
  <style>
    body { font-family: system-ui, sans-serif; margin: 0; background: #0f172a; color: #e2e8f0; }
    header, main { max-width: 1200px; margin: 0 auto; padding: 1rem; }
    input, button, select { border: 1px solid #475569; border-radius: .5rem; padding: .6rem; background: #020617; color: #e2e8f0; }
    button { cursor: pointer; background: #1d4ed8; border-color: #2563eb; }
    button:disabled { cursor: not-allowed; opacity: .45; }
    .search { display: grid; grid-template-columns: 2fr 1fr 1fr auto; gap: .5rem; }
    .pager { display: flex; align-items: center; gap: .5rem; margin: .5rem 0 1rem; }
    .layout { display: grid; grid-template-columns: 240px minmax(0, 1fr) minmax(320px, 480px); gap: 1rem; margin-top: 1rem; }
    .card { background: #111827; border: 1px solid #334155; border-radius: .75rem; padding: .8rem; margin-bottom: .6rem; }
    .row { cursor: pointer; }
    .row:hover { border-color: #60a5fa; }
    .meta, .tags, .sources { color: #94a3b8; font-size: .9rem; overflow-wrap: anywhere; }
    .tag, .facet { display: inline-block; margin: .15rem; padding: .15rem .4rem; border-radius: 999px; background: #1e293b; color: #bfdbfe; }
    .facet { cursor: pointer; border: 1px solid #334155; }
    .facet:hover { border-color: #60a5fa; }
    pre { white-space: pre-wrap; overflow-wrap: anywhere; background: #020617; border-radius: .5rem; padding: .75rem; }
    a { color: #93c5fd; }
  </style>
</head>
<body>
  <header>
    <h1>FLETCH Registry Search</h1>
    <div id="summary" class="meta">Loading index summary...</div>
  </header>
  <main>
    <form id="search" class="search">
      <input id="text" name="text" placeholder="Multi-term text: storm foundation, MIT algorithms..." autofocus />
      <input id="tag" name="tag" placeholder="Tags, comma separated" />
      <input id="metadata" name="metadata" placeholder="Metadata filters, comma separated key=value" />
      <button type="submit">Search</button>
    </form>
    <div class="layout">
      <nav>
        <h2>Facets</h2>
        <div id="facets" class="card meta">Loading facets...</div>
      </nav>
      <section>
        <div id="result-count" class="meta"></div>
        <div class="pager">
          <button type="button" id="prev-page">Previous page</button>
          <label class="meta">Sort
            <select id="sort">
              <option value="index" selected>Index order</option>
              <option value="fletch_id">Fletch ID</option>
              <option value="registry_id">Registry ID</option>
              <option value="owner_repo">Owner repo</option>
              <option value="domain">Domain</option>
              <option value="asset_kind">Asset kind</option>
              <option value="node_kind">Node kind</option>
            </select>
          </label>
          <label class="meta">Direction
            <select id="direction">
              <option value="asc" selected>Asc</option>
              <option value="desc">Desc</option>
            </select>
          </label>
          <label class="meta">Page size
            <select id="page-size">
              <option value="25">25</option>
              <option value="50" selected>50</option>
              <option value="100">100</option>
            </select>
          </label>
          <button type="button" id="next-page">Next page</button>
        </div>
        <div id="results"></div>
      </section>
      <aside>
        <h2>Detail</h2>
        <div id="detail" class="card meta">Select a row to inspect tags, metadata, and source URLs.</div>
      </aside>
    </div>
  </main>
  <script>
    const results = document.querySelector('#results');
    const detail = document.querySelector('#detail');
    const count = document.querySelector('#result-count');
    const facets = document.querySelector('#facets');
    const prevPage = document.querySelector('#prev-page');
    const nextPage = document.querySelector('#next-page');
    const pageSize = document.querySelector('#page-size');
    const sort = document.querySelector('#sort');
    const direction = document.querySelector('#direction');
    let currentOffset = 0;
    let matchedRowCount = 0;

    function esc(value) {
      return String(value ?? '').replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
    }

    function rowCard(row) {
      const div = document.createElement('div');
      div.className = 'card row';
      div.innerHTML = `<strong>${esc(row.fletch_id)}</strong>
        <div class="meta">${esc(row.registry_id)} · ${esc(row.node_kind)}</div>
        <div class="tags">${(row.tags || []).map(tag => `<span class="tag">${esc(tag)}</span>`).join('')}</div>
        <div class="sources">${(row.source_urls || []).map(url => `<div>${esc(url)}</div>`).join('')}</div>`;
      div.addEventListener('click', () => showDetail(row));
      return div;
    }

    function showDetail(row) {
      const urls = (row.source_urls || []).map((url, index) => {
        const link = url.startsWith('http') ? `<a href="${esc(url)}" target="_blank" rel="noreferrer">${esc(url)}</a>` : esc(url);
        return `<li>${link} <button type="button" onclick="loadSource('${esc(row.registry_id)}','${esc(row.fletch_id)}',${index})">Load preview</button></li>`;
      }).join('');
      detail.dataset.registryId = row.registry_id;
      detail.dataset.fletchId = row.fletch_id;
      detail.dataset.sourceIndex = '0';
      detail.innerHTML = `<h3>${esc(row.fletch_id)}</h3>
        <div class="meta">${esc(row.registry_id)} · ${esc(row.node_kind)}</div>
        <h4>Sources</h4><ul>${urls}</ul>
        <h4>Tags</h4><div>${(row.tags || []).map(tag => `<span class="tag">${esc(tag)}</span>`).join('')}</div>
        <h4>Metadata</h4><pre>${esc(JSON.stringify(row.metadata || {}, null, 2))}</pre>
        <h4>Loaded source preview</h4>
        <div id="source-controls" class="meta"></div>
        <pre id="source-preview">Click "Load preview" beside a source URL to fetch bounded source data.</pre>`;
    }

    async function loadSource(registryId, fletchId, sourceIndex, lineStart = 1) {
      const preview = document.querySelector('#source-preview');
      const controls = document.querySelector('#source-controls');
      detail.dataset.registryId = registryId;
      detail.dataset.fletchId = fletchId;
      detail.dataset.sourceIndex = String(sourceIndex);
      preview.textContent = 'Loading source preview...';
      const params = new URLSearchParams({ registry_id: registryId, fletch_id: fletchId, source: String(sourceIndex), line_start: String(lineStart), line_count: '80' });
      const response = await fetch(`/api/source?${params}`);
      if (!response.ok) {
        preview.textContent = `Could not load source preview: ${response.status} ${response.statusText}`;
        return;
      }
      const data = await response.json();
      const next = data.line_start + data.line_count;
      const prev = Math.max(1, data.line_start - 80);
      const outline = data.json_outline ? `\nJSON outline: ${JSON.stringify(data.json_outline)}` : '';
      controls.innerHTML = `Resolved: <a href="${esc(data.resolved_url)}" target="_blank" rel="noreferrer">${esc(data.resolved_url)}</a><br>
        Bytes: ${data.byte_count}${data.truncated ? ' (truncated)' : ''} · Lines: ${data.total_line_count}${outline}<br>
        <button type="button" onclick="loadCurrentSource(${prev})">Previous lines</button>
        <button type="button" onclick="loadCurrentSource(${next})">Next lines</button>`;
      preview.textContent = data.lines.map(line => `${String(line.number).padStart(5, ' ')}  ${line.text}`).join('\n');
    }

    function loadCurrentSource(lineStart) {
      loadSource(detail.dataset.registryId, detail.dataset.fletchId, Number(detail.dataset.sourceIndex || '0'), lineStart);
    }

    function applyFacet(kind, value) {
      if (kind === 'tag') {
        document.querySelector('#tag').value = value;
      } else {
        const metadata = document.querySelector('#metadata');
        const filter = `${kind}=${value}`;
        metadata.value = metadata.value ? `${metadata.value},${filter}` : filter;
      }
      runSearch(undefined, 0);
    }

    function facetGroup(title, kind, items) {
      const chips = (items || []).slice(0, 20).map(item => `<span class="facet" onclick="applyFacet('${esc(kind)}','${esc(item.value)}')">${esc(item.value)} (${item.count})</span>`).join('');
      return `<h3>${esc(title)}</h3>${chips || '<div class="meta">No values</div>'}`;
    }

    async function runSearch(event, offset = 0) {
      event?.preventDefault();
      currentOffset = Math.max(0, offset);
      const params = new URLSearchParams();
      const text = document.querySelector('#text').value.trim();
      const tag = document.querySelector('#tag').value.trim();
      const metadata = document.querySelector('#metadata').value.trim();
      if (text) params.set('text', text);
      if (tag) tag.split(',').map(v => v.trim()).filter(Boolean).forEach(v => params.append('tag', v));
      if (metadata) metadata.split(',').map(v => v.trim()).filter(Boolean).forEach(v => params.append('metadata', v));
      params.set('offset', String(currentOffset));
      params.set('limit', pageSize.value);
      params.set('sort', sort.value);
      params.set('direction', direction.value);
      const response = await fetch(`/api/search?${params}`);
      const report = await response.json();
      matchedRowCount = report.matched_row_count;
      const first = matchedRowCount === 0 ? 0 : currentOffset + 1;
      const last = currentOffset + report.rows.length;
      count.textContent = `${matchedRowCount} matches (${first}-${last} shown)`;
      prevPage.disabled = currentOffset === 0;
      nextPage.disabled = currentOffset + report.rows.length >= matchedRowCount;
      results.replaceChildren(...report.rows.map(rowCard));
      if (report.rows[0]) {
        showDetail(report.rows[0]);
      } else {
        detail.textContent = 'No matching rows.';
      }
    }

    fetch('/api/summary').then(r => r.json()).then(summary => {
      document.querySelector('#summary').textContent = `${summary.registry_count} registries · ${summary.row_count} rows · ${summary.fletch_count} unique fletches`;
    });
    fetch('/api/facets').then(r => r.json()).then(data => {
      facets.innerHTML = [
        facetGroup('Owner repos', 'owner_repo', data.metadata.owner_repo),
        facetGroup('Domains', 'domain', data.metadata.domain),
        facetGroup('Asset kinds', 'asset_kind', data.metadata.asset_kind),
        facetGroup('Fetch policy', 'fetch_policy', data.metadata.fetch_policy),
        facetGroup('Tags', 'tag', data.tags)
      ].join('');
    });
    document.querySelector('#search').addEventListener('submit', event => runSearch(event, 0));
    prevPage.addEventListener('click', () => runSearch(undefined, currentOffset - Number(pageSize.value || '50')));
    nextPage.addEventListener('click', () => runSearch(undefined, currentOffset + Number(pageSize.value || '50')));
    pageSize.addEventListener('change', () => runSearch(undefined, 0));
    sort.addEventListener('change', () => runSearch(undefined, 0));
    direction.addEventListener('change', () => runSearch(undefined, 0));
    runSearch();
  </script>
</body>
</html>
"#;

fn expected_dataset_ids_from_inputs(
    explicit_ids: Vec<String>,
    registry_paths: Vec<PathBuf>,
) -> Result<Vec<String>> {
    let mut expected = explicit_ids.into_iter().collect::<BTreeSet<_>>();
    for registry_path in registry_paths {
        let registry = read_registry(&registry_path)?;
        expected.extend(
            registry
                .fletches
                .into_iter()
                .filter(|definition| {
                    definition
                        .shafts
                        .iter()
                        .any(|shaft| matches!(shaft.kind, SourceKind::Http | SourceKind::File))
                })
                .map(|definition| definition.id),
        );
    }
    Ok(expected.into_iter().collect())
}

fn read_crop_index(path: &PathBuf) -> Result<CropIndexReport> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn read_cache_index(path: &PathBuf) -> Result<CacheIndexReport> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn read_proof_docs(path: &PathBuf) -> Result<ProofDocumentManifest> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn read_local_url_map(path: &PathBuf) -> Result<LocalUrlMap> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn read_quiver_summary(path: &PathBuf) -> Result<QuiverSummary> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn read_adapter_handoff(path: &PathBuf) -> Result<AdapterHandoffReport> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn read_quiver_manifest(path: &PathBuf) -> Result<QuiverManifest> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn read_alias_state(path: &PathBuf) -> Result<AliasState> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn read_label_state(path: &PathBuf) -> Result<LabelState> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn read_partition_state(path: &PathBuf) -> Result<PartitionState> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn read_rollup_preview(path: &PathBuf) -> Result<RollupPreview> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn write_fetch_manifest(
    cache_root: &PathBuf,
    entry: CacheEntry,
    output: Option<PathBuf>,
) -> Result<()> {
    let cache_root = cache_root.display().to_string();
    let manifest = if let Some(output) = output.as_ref().filter(|path| path.exists()) {
        let manifest = read_manifest(output)?;
        if manifest.cache_root != cache_root {
            bail!(
                "output manifest cache root {} does not match requested cache root {}",
                manifest.cache_root,
                cache_root
            );
        }
        upsert_cache_manifest_entries(manifest, [entry])?
    } else {
        cache_manifest(cache_root, vec![entry])?
    };
    if let Some(output) = output {
        write_cache_manifest_json(output, &manifest)?;
        Ok(())
    } else {
        write_json(&manifest, None)
    }
}

fn write_json<T: serde::Serialize + ?Sized>(value: &T, output: Option<PathBuf>) -> Result<()> {
    let json = serde_json::to_string_pretty(value)?;
    if let Some(output) = output {
        fs::write(output, json)?;
    } else {
        println!("{json}");
    }
    Ok(())
}
