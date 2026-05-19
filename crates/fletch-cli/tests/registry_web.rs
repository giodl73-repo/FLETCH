use std::error::Error;
use std::fs;
use std::net::{SocketAddr, TcpListener};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

struct WebServer {
    child: Child,
}

impl Drop for WebServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[test]
fn registry_web_serves_summary_search_detail_and_html() -> Result<(), Box<dyn Error>> {
    let (index_path, source_path) = write_test_index()?;
    let port = available_port()?;
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let server = WebServer {
        child: Command::new(env!("CARGO_BIN_EXE_fletch-cli"))
            .args([
                "registry",
                "web",
                "--index",
                index_path.to_str().expect("temp path should be UTF-8"),
                "--port",
                &port.to_string(),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?,
    };

    wait_for_server(address)?;

    let html = http_get(address, "/")?;
    assert!(html.contains("FLETCH Registry Search"));
    assert!(html.contains("Next page"));
    assert!(html.contains("Owner repo"));
    assert!(html.contains("Copy link"));
    assert!(html.contains("Export CSV"));
    assert!(html.contains("Export all CSV"));
    assert!(html.contains("Relevance"));
    assert!(html.contains("Loading presets"));
    assert!(html.contains("highlightSnippet"));
    assert!(html.contains("currentTextSearchTerms"));
    assert!(html.contains("Copy row link"));
    assert!(html.contains("selectedRegistryFromUrl"));

    let linked_html = http_get(
        address,
        "/?text=storm%20seed&sort=fletch_id&direction=desc&offset=1&limit=25&selected_registry_id=storm-foundation-assets&selected_fletch_id=storm.foundation.seed-storm",
    )?;
    assert!(linked_html.contains("loadControlsFromUrl"));
    assert!(linked_html.contains("loadSelectedRowDetail"));

    let summary = http_get(address, "/api/summary")?;
    assert!(summary.contains("\"row_count\": 2"));

    let facets = http_get(address, "/api/facets")?;
    assert!(facets.contains("\"owner_repo\""));
    assert!(facets.contains("\"STORM\""));
    assert!(facets.contains("\"storm\""));

    let presets = http_get(address, "/api/presets")?;
    assert!(presets.contains("\"mit-textbooks\""));
    assert!(presets.contains("\"repo-registries\""));

    let search = http_get(address, "/api/search?text=storm%20seed&limit=10")?;
    assert!(search.contains("\"matched_row_count\": 1"));
    assert!(search.contains("\"storm.foundation.seed-storm\""));
    assert!(search.contains("\"weather\""));
    assert!(search.contains("\"snippets\""));
    assert!(search.contains("\"scores\""));
    assert!(search.contains("fletch_id: storm.foundation.seed-storm"));
    assert!(search.contains("\"matched_terms\""));

    let export = http_get(address, "/api/export.csv?text=storm%20seed&limit=10")?;
    assert!(export.contains("registry_id,fletch_id,node_kind,score,snippet"));
    assert!(export.contains("storm-foundation-assets,storm.foundation.seed-storm"));

    let export_page = http_get(address, "/api/export.csv?limit=1")?;
    assert!(export_page.contains("storm-foundation-assets,storm.foundation.seed-storm"));
    assert!(!export_page.contains("mundus-knowledge-systems-registries,mundus.registry.porto"));
    let export_all = http_get(address, "/api/export.csv?limit=1&all=true")?;
    assert!(export_all.contains("storm-foundation-assets,storm.foundation.seed-storm"));
    assert!(export_all.contains("mundus-knowledge-systems-registries,mundus.registry.porto"));

    let paged = http_get(address, "/api/search?offset=1&limit=1")?;
    assert!(paged.contains("\"matched_row_count\": 2"));
    assert!(paged.contains("\"mundus.registry.porto\""));

    let sorted = http_get(address, "/api/search?sort=fletch_id&direction=asc&limit=1")?;
    assert!(sorted.contains("\"matched_row_count\": 2"));
    assert!(sorted.contains("\"mundus.registry.porto\""));
    assert!(!sorted.contains("\"storm.foundation.seed-storm\""));

    let detail = http_get(
        address,
        "/api/row?registry_id=storm-foundation-assets&fletch_id=storm.foundation.seed-storm",
    )?;
    assert!(detail.contains("\"source_urls\""));
    assert!(detail.contains("fletch-registry-web-source"));

    let source = http_get(
        address,
        "/api/source?registry_id=storm-foundation-assets&fletch_id=storm.foundation.seed-storm&source=0&line_start=1&line_count=1",
    )?;
    assert!(source.contains("storm fixture payload"));
    assert!(source.contains("\"truncated\": false"));
    assert!(source.contains("\"total_line_count\""));
    assert!(source.contains("\"json_outline\""));

    drop(server);
    fs::remove_file(index_path)?;
    fs::remove_file(source_path)?;
    Ok(())
}

#[test]
fn registry_web_can_build_index_from_registry_files() -> Result<(), Box<dyn Error>> {
    let registry_path = write_test_registry()?;
    let port = available_port()?;
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let server = WebServer {
        child: Command::new(env!("CARGO_BIN_EXE_fletch-cli"))
            .args([
                "registry",
                "web",
                "--file",
                registry_path.to_str().expect("temp path should be UTF-8"),
                "--port",
                &port.to_string(),
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?,
    };

    wait_for_server(address)?;

    let summary = http_get(address, "/api/summary")?;
    assert!(summary.contains("\"registry_count\": 1"));
    assert!(summary.contains("\"row_count\": 1"));

    let search = http_get(address, "/api/search?text=direct%20registry&limit=10")?;
    assert!(search.contains("\"matched_row_count\": 1"));
    assert!(search.contains("\"test.direct.registry\""));

    drop(server);
    fs::remove_file(registry_path)?;
    Ok(())
}

fn write_test_index() -> Result<(std::path::PathBuf, std::path::PathBuf), Box<dyn Error>> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let path = std::env::temp_dir().join(format!(
        "fletch-registry-web-test-{}-{stamp}.json",
        std::process::id()
    ));
    let source_path = std::env::temp_dir().join(format!(
        "fletch-registry-web-source-{}-{stamp}.json",
        std::process::id()
    ));
    fs::write(&source_path, r#"{"name":"storm fixture payload"}"#)?;
    let index = serde_json::json!({
        "schema_version": "fletch.registry-index.v1",
        "generated_by": "test",
        "registry_count": 1,
        "fletch_count": 2,
        "row_count": 2,
        "rows": [
            {
                "registry_id": "storm-foundation-assets",
                "fletch_id": "storm.foundation.seed-storm",
                "node_kind": "document",
                "source_urls": [source_path.to_string_lossy()],
                "source_kinds": ["file"],
                "tags": ["storm", "weather", "hazard", "fixture", "seed"],
                "metadata": {
                    "owner_repo": "STORM",
                    "asset_kind": "seed-fixture",
                    "fetch_policy": "local_file"
                }
            },
            {
                "registry_id": "mundus-knowledge-systems-registries",
                "fletch_id": "mundus.registry.porto",
                "node_kind": "fletch",
                "source_urls": ["https://raw.githubusercontent.com/giodl73-repo/PORTO/main/.fletch/registries/porto-ash-vale-assets.json"],
                "source_kinds": ["http"],
                "tags": ["mundus", "repo-registry", "knowledge-systems", "porto"],
                "metadata": {
                    "owner_repo": "PORTO",
                    "fetch_policy": "metadata_only"
                }
            }
        ]
    });
    fs::write(&path, serde_json::to_string_pretty(&index)?)?;
    Ok((path, source_path))
}

fn write_test_registry() -> Result<std::path::PathBuf, Box<dyn Error>> {
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let path = std::env::temp_dir().join(format!(
        "fletch-registry-web-direct-{}-{stamp}.json",
        std::process::id()
    ));
    let registry = serde_json::json!({
        "schema_version": "fletch.registry.v1",
        "generated_by": "test",
        "registry_id": "direct-registry-test",
        "fletches": [
            {
                "id": "test.direct.registry",
                "node_kind": "document",
                "shafts": [{"kind": "file", "url": "direct-registry.json"}],
                "tags": ["direct", "registry"],
                "metadata": {
                    "owner_repo": "TEST",
                    "asset_kind": "direct-registry"
                }
            }
        ]
    });
    fs::write(&path, serde_json::to_string_pretty(&registry)?)?;
    Ok(path)
}

fn available_port() -> Result<u16, Box<dyn Error>> {
    let listener = TcpListener::bind(("127.0.0.1", 0))?;
    let port = listener.local_addr()?.port();
    drop(listener);
    Ok(port)
}

fn wait_for_server(address: SocketAddr) -> Result<(), Box<dyn Error>> {
    for _ in 0..50 {
        if http_get(address, "/api/summary").is_ok() {
            return Ok(());
        }
        thread::sleep(Duration::from_millis(100));
    }
    Err("registry web server did not start".into())
}

fn http_get(address: SocketAddr, path: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()?
        .get(format!("http://{address}{path}"))
        .send()?
        .error_for_status()?;
    Ok(response.text()?)
}
