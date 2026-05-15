use anyhow::Result;
use clap::{Parser, Subcommand};
use fletch_core::{cache_key, fetch_plan};
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
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Plan {
            dataset_id,
            url,
            output,
        } => {
            let plan = fetch_plan(dataset_id, url)?;
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
    }
    Ok(())
}
