use anyhow::Result;
use fletch_mock_client::run_mock_client;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<()> {
    let root = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(default_workspace_root);
    let report = run_mock_client(root)?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

fn default_workspace_root() -> PathBuf {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or_default();
    std::env::temp_dir().join(format!(
        "fletch-mock-client-{}-{millis}",
        std::process::id()
    ))
}
