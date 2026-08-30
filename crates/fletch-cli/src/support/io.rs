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
use std::path::{Path, PathBuf};

pub(crate) fn parse_headers(headers: Vec<String>) -> Result<BTreeMap<String, String>> {
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

pub(crate) fn parse_key_value_filters(filters: Vec<String>) -> Result<Vec<(String, String)>> {
    let mut parsed = Vec::new();
    for filter in filters {
        parsed.push(parse_key_value_filter(&filter)?);
    }
    Ok(parsed)
}

pub(crate) fn parse_key_value_filter(filter: &str) -> Result<(String, String)> {
    let Some((name, value)) = filter.split_once('=') else {
        bail!("filter must be formatted as name=value: {filter}");
    };
    let name = name.trim();
    if name.is_empty() {
        bail!("filter name must not be empty: {filter}");
    }
    Ok((name.to_string(), value.to_string()))
}

pub(crate) fn read_manifest(path: &PathBuf) -> Result<CacheManifest> {
    Ok(read_cache_manifest_json(path)?)
}

pub(crate) fn read_plan(path: &PathBuf) -> Result<FetchPlan> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

pub(crate) fn read_registry(path: &PathBuf) -> Result<FletchRegistry> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

pub(crate) fn read_registry_inputs(files: &[PathBuf], follow: bool) -> Result<Vec<FletchRegistry>> {
    let mut registries = files
        .iter()
        .map(read_registry)
        .collect::<Result<Vec<_>>>()?;
    if follow {
        let followed = follow_registry_pointers(&registries)?;
        registries.extend(followed);
    }
    ensure_valid_registry_inputs(&registries)?;
    Ok(registries)
}

fn ensure_valid_registry_inputs(registries: &[FletchRegistry]) -> Result<()> {
    let invalid = registries
        .iter()
        .filter_map(|registry| {
            let report = validate_registry(registry);
            if report.valid {
                return None;
            }
            let codes = report
                .findings
                .iter()
                .map(|finding| finding.code.as_str())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>()
                .join(",");
            Some(format!("{} [{codes}]", registry.registry_id))
        })
        .collect::<Vec<_>>();
    if !invalid.is_empty() {
        bail!("registry validation failed: {}", invalid.join("; "));
    }
    Ok(())
}

pub(crate) fn follow_registry_pointers(
    registries: &[FletchRegistry],
) -> Result<Vec<FletchRegistry>> {
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

pub(crate) fn fetch_registry_url(
    client: &reqwest::blocking::Client,
    url: &str,
) -> Result<FletchRegistry> {
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

pub(crate) fn read_registry_index(path: &PathBuf) -> Result<RegistryIndexReport> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

pub(crate) fn read_registry_web_index(
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

pub(crate) fn parse_query(query: &str) -> BTreeMap<String, Vec<String>> {
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

pub(crate) fn load_registry_source_preview(
    row: &RegistryIndexRow,
    source_index: usize,
    line_start: Option<usize>,
    line_count: usize,
    text: Option<&str>,
    matched_only: bool,
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
    let preview_text = String::from_utf8_lossy(preview_bytes).into_owned();
    let lines = preview_text.lines().collect::<Vec<_>>();
    let total_line_count = lines.len();
    let effective_line_count = line_count.min(500);
    let terms = registry_web_search_terms(text);
    let (start, matched_line) =
        source_preview_start(&lines, line_start, effective_line_count, &terms);
    let mut selected_lines = lines
        .iter()
        .enumerate()
        .skip(start.saturating_sub(1))
        .take(effective_line_count)
        .map(|(index, line)| {
            serde_json::json!({
                "number": index + 1,
                "text": line,
                "matched": source_line_matches_terms(line, &terms)
            })
        })
        .collect::<Vec<_>>();
    let matched_line_count = selected_lines
        .iter()
        .filter(|line| {
            line.get("matched")
                .and_then(|value| value.as_bool())
                .unwrap_or(false)
        })
        .count();
    if matched_only {
        selected_lines.retain(|line| {
            line.get("matched")
                .and_then(|value| value.as_bool())
                .unwrap_or(false)
        });
    }
    Ok(serde_json::json!({
        "boundary_notice": REGISTRY_WEB_BOUNDARY_NOTICE,
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
        "matched_line": matched_line,
        "matched_terms": terms,
        "matched_line_count": matched_line_count,
        "matched_only": matched_only,
        "line_count": selected_lines.len(),
        "lines": selected_lines,
        "json_outline": json_outline(&preview_text),
        "text": preview_text
    }))
}

pub(crate) fn write_json_response<T: serde::Serialize + ?Sized>(
    stream: &mut TcpStream,
    value: &T,
) -> Result<()> {
    let body = serde_json::to_string_pretty(value)?;
    write_http_response(stream, "200 OK", "application/json; charset=utf-8", &body)
}

pub(crate) fn write_http_response(
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

pub(crate) fn read_mdcrop_index(path: &PathBuf) -> Result<MdcropIndexReport> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

pub(crate) fn read_cache_index(path: &PathBuf) -> Result<CacheIndexReport> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

pub(crate) fn read_proof_docs(path: &PathBuf) -> Result<ProofDocumentManifest> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

pub(crate) fn read_local_url_map(path: &PathBuf) -> Result<LocalUrlMap> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

pub(crate) fn read_quiver_summary(path: &PathBuf) -> Result<QuiverSummary> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

pub(crate) fn read_adapter_handoff(path: &PathBuf) -> Result<AdapterHandoffReport> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

pub(crate) fn read_quiver_manifest(path: &PathBuf) -> Result<QuiverManifest> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

pub(crate) fn read_alias_state(path: &PathBuf) -> Result<AliasState> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

pub(crate) fn read_label_state(path: &PathBuf) -> Result<LabelState> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

pub(crate) fn read_partition_state(path: &PathBuf) -> Result<PartitionState> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

pub(crate) fn read_rollup_preview(path: &PathBuf) -> Result<RollupPreview> {
    let json = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&json)?)
}

pub(crate) fn write_fetch_manifest(
    cache_root: &Path,
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

pub(crate) fn write_json<T: serde::Serialize + ?Sized>(
    value: &T,
    output: Option<PathBuf>,
) -> Result<()> {
    let json = serde_json::to_string_pretty(value)?;
    if let Some(output) = output {
        fs::write(output, json)?;
    } else {
        println!("{json}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_registry_inputs_are_not_indexed() {
        let registry = serde_json::from_str(
            r#"{
                "schema_version": "fletch.registry.v1",
                "generated_by": "fletch-cli test",
                "registry_id": "invalid-registry",
                "fletches": [{
                    "id": "invalid.missing-shaft",
                    "node_kind": "fletch",
                    "shafts": [],
                    "metadata": {},
                    "tags": []
                }]
            }"#,
        )
        .expect("registry fixture should parse");

        let error = ensure_valid_registry_inputs(&[registry])
            .expect_err("invalid registry should be rejected")
            .to_string();

        assert!(error.contains("invalid-registry [missing-shaft]"));
    }
}
