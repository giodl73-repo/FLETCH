use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use fletch_core::{
    cache_key, cache_list, cache_manifest, dry_run_flight, export_quiver, fetch_plan,
    fetch_plan_with_kind, fetch_to_cache, graph_from_manifest, graph_from_registry, import_quiver,
    inspect_cache_manifest, plan_cache_prune, tips_from_manifest, CacheManifest, FetchOptions,
    FletchRegistry, FreshnessPolicy, SourceKind,
};
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
        /// Cache root. Defaults to .fletch/cache.
        #[arg(long, default_value = ".fletch/cache")]
        cache_root: PathBuf,
        /// Expected sha256, formatted as sha256:<64 lowercase hex chars>.
        #[arg(long)]
        expect_sha256: Option<String>,
        /// Maximum transfer/write rate in bytes per second.
        #[arg(long)]
        max_bytes_per_second: Option<u64>,
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
            freshness,
            max_age_days,
            output,
        } => {
            let mut plan = fetch_plan_with_kind(dataset_id, url, source_kind.into())?;
            plan.cache_policy.freshness = freshness_policy(freshness, max_age_days)?;
            let json = serde_json::to_string_pretty(&plan)?;
            if let Some(output) = output {
                fs::write(output, json)?;
            } else {
                println!("{json}");
            }
        }
        Commands::Key { dataset_id, url } => {
            let plan = fetch_plan(dataset_id, url)?;
            println!("{}", cache_key(&plan));
        }
        Commands::Fetch {
            dataset_id,
            url,
            source_kind,
            cache_root,
            expect_sha256,
            max_bytes_per_second,
            force,
            offline,
            freshness,
            max_age_days,
            output,
        } => {
            let mut plan = fetch_plan_with_kind(dataset_id, url, source_kind.into())?;
            plan.cache_policy.freshness = freshness_policy(freshness, max_age_days)?;
            let mut options = FetchOptions::new(&cache_root)
                .with_force(force)
                .with_offline(offline);
            if let Some(expected) = expect_sha256 {
                options = options.with_expected_sha256(expected);
            }
            if let Some(max_bytes_per_second) = max_bytes_per_second {
                options = options.with_max_bytes_per_second(max_bytes_per_second);
            }
            let outcome = fetch_to_cache(&plan, options)?;
            let manifest = cache_manifest(cache_root.display().to_string(), vec![outcome.entry])?;
            let json = serde_json::to_string_pretty(&manifest)?;
            if let Some(output) = output {
                fs::write(output, json)?;
            } else {
                println!("{json}");
            }
        }
        Commands::Cache { command } => match command {
            CacheCommands::List { manifest, output } => {
                let manifest = read_manifest(&manifest)?;
                write_json(cache_list(&manifest), output)?;
            }
            CacheCommands::Verify { manifest, output } => {
                let manifest = read_manifest(&manifest)?;
                write_json(
                    &inspect_cache_manifest(&manifest, &FreshnessPolicy::Immutable)?,
                    output,
                )?;
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

fn read_manifest(path: &PathBuf) -> Result<CacheManifest> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

fn read_registry(path: &PathBuf) -> Result<FletchRegistry> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
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
