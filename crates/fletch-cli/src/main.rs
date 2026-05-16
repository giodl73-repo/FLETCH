use anyhow::{bail, Result};
use clap::{Parser, Subcommand, ValueEnum};
use fletch_core::{
    active_partition_set, adapter_handoff_report, adapter_sources_from_registry,
    alias_state_from_manifest, cache_key, cache_list, cache_manifest, crop_index_from_manifest,
    dry_run_flight, export_quiver, fetch_plan, fetch_plan_with_kind, fetch_to_cache,
    graph_from_manifest, graph_from_quiver, graph_from_registry, import_quiver,
    inspect_cache_manifest, label_state_from_aliases, local_url_map, offline_cache_report,
    partition_invalidation_report, partition_state_from_manifest, plan_cache_prune,
    preview_archive_expansion, preview_manifest_merge, preview_rollback, preview_rollup_edges,
    proof_document_manifest, publish_report_from_manifest, publisher_bundle_report,
    quiver_merge_ready_report, slice_active_partition_set, slice_adapter_source_report,
    slice_archive_expansion_preview, slice_crop_index_report, slice_local_url_map,
    slice_partition_state, slice_proof_document_manifest, slice_quiver_merge_ready_report,
    slice_registry_validation_report, summarize_cache_manifest, summarize_quiver,
    tips_from_manifest, upsert_cache_manifest_entry, validate_registry, verify_cache_manifest,
    verify_quiver_bundle, AdapterHandoffReport, AliasState, CacheEntry, CacheManifest,
    CropIndexReport, FetchOptions, FetchPlan, FletchRegistry, FreshnessPolicy, LabelState,
    LocalUrlMap, PartitionState, ProofDocumentManifest, QuiverManifest, QuiverSummary,
    RollupPreview, SourceKind,
};
use std::collections::BTreeMap;
use std::fs;
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

fn read_manifest(path: &PathBuf) -> Result<CacheManifest> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn read_plan(path: &PathBuf) -> Result<FetchPlan> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn read_registry(path: &PathBuf) -> Result<FletchRegistry> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn read_crop_index(path: &PathBuf) -> Result<CropIndexReport> {
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
        upsert_cache_manifest_entry(manifest, entry)?
    } else {
        cache_manifest(cache_root, vec![entry])?
    };
    write_json(&manifest, output)
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
