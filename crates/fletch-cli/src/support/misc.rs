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

pub(crate) fn freshness_policy(
    freshness: CliFreshness,
    max_age_days: Option<u32>,
) -> Result<FreshnessPolicy> {
    match freshness {
        CliFreshness::Immutable => Ok(FreshnessPolicy::Immutable),
        CliFreshness::AlwaysCheck => Ok(FreshnessPolicy::AlwaysCheck),
        CliFreshness::MaxAgeDays => Ok(FreshnessPolicy::MaxAgeDays(
            max_age_days.ok_or_else(|| anyhow::anyhow!("--max-age-days is required"))?,
        )),
    }
}

pub(crate) fn registry_urls_from_pointer(
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

pub(crate) fn registry_urls_from_github_contents(
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

pub(crate) fn github_contents_download_url(value: &serde_json::Value) -> Option<String> {
    let name = value.get("name")?.as_str()?;
    if !name.ends_with(".json") {
        return None;
    }
    value
        .get("download_url")
        .and_then(|download_url| download_url.as_str())
        .map(ToString::to_string)
}

pub(crate) fn annotate_registry_source(registry: &mut FletchRegistry, url: &str) {
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

pub(crate) fn raw_github_repo_base_url(url: &str) -> Option<String> {
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

pub(crate) fn github_tree_url_to_contents_api(url: &str) -> Option<String> {
    github_url_to_contents_api(url, "tree")
}

pub(crate) fn github_blob_url_to_raw(url: &str) -> Option<String> {
    let (owner, repo, branch, path) = github_url_parts(url, "blob")?;
    Some(format!(
        "https://raw.githubusercontent.com/{owner}/{repo}/{branch}/{path}"
    ))
}

pub(crate) fn github_url_to_contents_api(url: &str, marker: &str) -> Option<String> {
    let (owner, repo, branch, path) = github_url_parts(url, marker)?;
    Some(format!(
        "https://api.github.com/repos/{owner}/{repo}/contents/{path}?ref={branch}"
    ))
}

pub(crate) fn github_url_parts(
    url: &str,
    marker: &str,
) -> Option<(String, String, String, String)> {
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

pub(crate) fn serve_registry_web(
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

pub(crate) fn open_browser(url: &str) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        let mut command = std::process::Command::new("cmd");
        command.args(["/C", "start", "", url]);
        run_browser_launcher(command)
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

pub(crate) fn handle_registry_web_request(
    mut stream: TcpStream,
    index: &RegistryIndexReport,
) -> Result<()> {
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
        "/api/presets" => write_json_response(&mut stream, &registry_web_presets()),
        "/api/search" => {
            let query = parse_query(query);
            let (report, text) = registry_web_search_from_query(index, &query, false)?;
            write_json_response(
                &mut stream,
                &registry_web_search_response(&report, text.as_deref())?,
            )
        }
        "/api/export.csv" => {
            let query = parse_query(query);
            let all_rows = query_flag(&query, "all");
            let (report, text) = registry_web_search_from_query(index, &query, all_rows)?;
            write_http_response(
                &mut stream,
                "200 OK",
                "text/csv; charset=utf-8",
                &registry_web_search_csv(&report, text.as_deref())?,
            )
        }
        "/api/export.json" => {
            let query = parse_query(query);
            let all_rows = query_flag(&query, "all");
            let (report, text) = registry_web_search_from_query(index, &query, all_rows)?;
            write_json_response(
                &mut stream,
                &registry_web_search_response(&report, text.as_deref())?,
            )
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
                .and_then(|value| value.parse::<usize>().ok());
            let line_count = query
                .get("line_count")
                .and_then(|values| values.first())
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(80);
            let text = query.get("text").and_then(|values| values.first());
            let matched_only = query_flag(&query, "matched_only");
            let row = registry_id
                .zip(fletch_id)
                .and_then(|(registry_id, fletch_id)| {
                    find_registry_index_row(index, registry_id, fletch_id)
                });
            if let Some(row) = row {
                write_json_response(
                    &mut stream,
                    &load_registry_source_preview(
                        row,
                        source_index,
                        line_start,
                        line_count,
                        text.map(String::as_str),
                        matched_only,
                    )?,
                )
            } else {
                write_http_response(&mut stream, "404 Not Found", "text/plain", "row not found")
            }
        }
        _ => write_http_response(&mut stream, "404 Not Found", "text/plain", "not found"),
    }
}

pub(crate) fn split_path_query(target: &str) -> (&str, &str) {
    target.split_once('?').unwrap_or((target, ""))
}

pub(crate) fn url_decode(value: &str) -> String {
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

pub(crate) fn find_registry_index_row<'a>(
    index: &'a RegistryIndexReport,
    registry_id: &str,
    fletch_id: &str,
) -> Option<&'a RegistryIndexRow> {
    index
        .rows
        .iter()
        .find(|row| row.registry_id == registry_id && row.fletch_id == fletch_id)
}

pub(crate) fn registry_web_facets(index: &RegistryIndexReport) -> serde_json::Value {
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

pub(crate) fn registry_web_presets() -> serde_json::Value {
    serde_json::json!([
        {
            "id": "mit-textbooks",
            "label": "MIT textbooks",
            "text": "MIT textbook",
            "tag": "known-asset",
            "metadata": "fetch_policy=metadata_only",
            "sort": "relevance",
            "direction": "desc"
        },
        {
            "id": "repo-registries",
            "label": "Knowledge repo registries",
            "text": "",
            "tag": "repo-registry",
            "metadata": "fetch_policy=metadata_only",
            "sort": "owner_repo",
            "direction": "asc"
        },
        {
            "id": "source-corpus",
            "label": "Source corpus packs",
            "text": "source corpus pack",
            "tag": "",
            "metadata": "",
            "sort": "relevance",
            "direction": "desc"
        },
        {
            "id": "storm-hazards",
            "label": "STORM hazards",
            "text": "storm hazard",
            "tag": "storm",
            "metadata": "owner_repo=STORM",
            "sort": "relevance",
            "direction": "desc"
        }
    ])
}

pub(crate) fn increment_facet(counts: &mut BTreeMap<String, usize>, value: &str) {
    *counts.entry(value.to_string()).or_insert(0) += 1;
}

pub(crate) fn top_facets(counts: BTreeMap<String, usize>, limit: usize) -> Vec<serde_json::Value> {
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

pub(crate) fn registry_web_search_from_query(
    index: &RegistryIndexReport,
    query: &BTreeMap<String, Vec<String>>,
    all_rows: bool,
) -> Result<(fletch_core::RegistrySearchReport, Option<String>)> {
    let tags = query.get("tag").cloned().unwrap_or_default();
    let metadata = query
        .get("metadata")
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|filter| parse_key_value_filter(&filter))
        .collect::<Result<Vec<_>>>()?;
    let text = query.get("text").and_then(|values| values.first()).cloned();
    let offset = if all_rows {
        0
    } else {
        query
            .get("offset")
            .and_then(|values| values.first())
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(0)
    };
    let limit = if all_rows {
        None
    } else {
        query
            .get("limit")
            .and_then(|values| values.first())
            .and_then(|value| value.parse::<usize>().ok())
            .or(Some(50))
    };
    let sort = query
        .get("sort")
        .and_then(|values| values.first())
        .map(String::as_str);
    let direction = query
        .get("direction")
        .and_then(|values| values.first())
        .map(String::as_str);
    let report = search_registry_web_index(
        index,
        RegistryWebSearchOptions {
            tags: &tags,
            metadata_filters: &metadata,
            text: text.as_deref(),
            offset,
            limit,
            sort,
            direction,
        },
    );
    Ok((report, text))
}

pub(crate) fn query_flag(query: &BTreeMap<String, Vec<String>>, key: &str) -> bool {
    query.get(key).is_some_and(|values| {
        values
            .iter()
            .any(|value| matches!(value.as_str(), "1" | "true" | "yes" | "on"))
    })
}

pub(crate) struct RegistryWebSearchOptions<'a> {
    pub(crate) tags: &'a [String],
    pub(crate) metadata_filters: &'a [(String, String)],
    pub(crate) text: Option<&'a str>,
    pub(crate) offset: usize,
    pub(crate) limit: Option<usize>,
    pub(crate) sort: Option<&'a str>,
    pub(crate) direction: Option<&'a str>,
}

pub(crate) fn search_registry_web_index(
    index: &RegistryIndexReport,
    options: RegistryWebSearchOptions<'_>,
) -> fletch_core::RegistrySearchReport {
    let mut report = search_registry_index(
        index,
        options.tags,
        options.metadata_filters,
        options.text,
        0,
        None,
    );
    let sort = options.sort.unwrap_or("relevance");
    let descending = options
        .direction
        .map_or(sort == "relevance", |direction| direction == "desc");
    sort_registry_web_rows(&mut report.rows, sort, descending, options.text);
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
        .skip(options.offset)
        .take(options.limit.unwrap_or(usize::MAX))
        .cloned()
        .collect();
    report
}

pub(crate) fn sort_registry_web_rows(
    rows: &mut [RegistryIndexRow],
    sort: &str,
    descending: bool,
    text: Option<&str>,
) {
    match sort {
        "relevance" => {
            let terms = registry_web_search_terms(text);
            if !terms.is_empty() {
                rows.sort_by(|left, right| {
                    let left_score = registry_web_relevance_score(left, &terms);
                    let right_score = registry_web_relevance_score(right, &terms);
                    if descending {
                        right_score
                            .cmp(&left_score)
                            .then_with(|| left.fletch_id.cmp(&right.fletch_id))
                    } else {
                        left_score
                            .cmp(&right_score)
                            .then_with(|| left.fletch_id.cmp(&right.fletch_id))
                    }
                });
            }
        }
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
    if descending && !matches!(sort, "relevance" | "owner_repo" | "domain" | "asset_kind") {
        rows.reverse();
    }
}

pub(crate) fn metadata_sort_order(
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

pub(crate) fn registry_web_search_response(
    report: &fletch_core::RegistrySearchReport,
    text: Option<&str>,
) -> Result<serde_json::Value> {
    let mut value = serde_json::to_value(report)?;
    if let serde_json::Value::Object(object) = &mut value {
        let terms = registry_web_search_terms(text);
        object.insert(
            "snippets".to_string(),
            serde_json::Value::Array(
                report
                    .rows
                    .iter()
                    .map(|row| registry_web_row_snippet(row, text))
                    .collect(),
            ),
        );
        object.insert(
            "scores".to_string(),
            serde_json::Value::Array(
                report
                    .rows
                    .iter()
                    .map(|row| serde_json::json!(registry_web_relevance_score(row, &terms)))
                    .collect(),
            ),
        );
    }
    Ok(value)
}

pub(crate) fn registry_web_search_csv(
    report: &fletch_core::RegistrySearchReport,
    text: Option<&str>,
) -> Result<String> {
    let terms = registry_web_search_terms(text);
    let mut csv =
        String::from("registry_id,fletch_id,node_kind,score,snippet,tags,source_urls,metadata\n");
    for row in &report.rows {
        let snippet = registry_web_row_snippet(row, text);
        let snippet_text = snippet
            .get("text")
            .and_then(|value| value.as_str())
            .unwrap_or_default();
        let values = [
            row.registry_id.clone(),
            row.fletch_id.clone(),
            format!("{:?}", row.node_kind),
            registry_web_relevance_score(row, &terms).to_string(),
            snippet_text.to_string(),
            row.tags.join("|"),
            row.source_urls.join("|"),
            serde_json::to_string(&row.metadata)?,
        ];
        csv.push_str(
            &values
                .iter()
                .map(|value| csv_escape(value))
                .collect::<Vec<_>>()
                .join(","),
        );
        csv.push('\n');
    }
    Ok(csv)
}

pub(crate) fn csv_escape(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') || value.contains('\r') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_string()
    }
}

pub(crate) fn registry_web_row_snippet(
    row: &RegistryIndexRow,
    text: Option<&str>,
) -> serde_json::Value {
    let terms = registry_web_search_terms(text);
    let fields = registry_web_snippet_fields(row);
    let selected = fields
        .iter()
        .find(|(_, value)| registry_web_snippet_matches(value, &terms))
        .or_else(|| fields.first());
    let (field, value) = selected
        .map(|(field, value)| (field.as_str(), value.as_str()))
        .unwrap_or(("fletch_id", row.fletch_id.as_str()));
    let matched_terms = terms
        .iter()
        .filter(|term| value.to_lowercase().contains(term.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    serde_json::json!({
        "field": field,
        "text": format!("{field}: {value}"),
        "matched_terms": matched_terms
    })
}

pub(crate) fn registry_web_search_terms(text: Option<&str>) -> Vec<String> {
    text.unwrap_or_default()
        .split_whitespace()
        .map(|term| term.to_lowercase())
        .collect()
}

pub(crate) fn registry_web_relevance_score(row: &RegistryIndexRow, terms: &[String]) -> usize {
    if terms.is_empty() {
        return 0;
    }
    let fields = registry_web_snippet_fields(row);
    fields
        .iter()
        .map(|(field, value)| {
            let lower = value.to_lowercase();
            terms
                .iter()
                .filter(|term| lower.contains(term.as_str()))
                .map(|term| registry_web_relevance_weight(field, term, &lower))
                .sum::<usize>()
        })
        .sum()
}

pub(crate) fn registry_web_relevance_weight(field: &str, term: &str, value: &str) -> usize {
    let base = match field {
        "fletch_id" => 12,
        "tag" => 8,
        field if field.starts_with("metadata.") => 6,
        "registry_id" => 5,
        "source_url" => 2,
        _ => 1,
    };
    if value == term {
        base * 3
    } else {
        base
    }
}

pub(crate) fn registry_web_snippet_fields(row: &RegistryIndexRow) -> Vec<(String, String)> {
    let mut fields = vec![
        ("fletch_id".to_string(), row.fletch_id.clone()),
        ("registry_id".to_string(), row.registry_id.clone()),
        ("node_kind".to_string(), format!("{:?}", row.node_kind)),
    ];
    fields.extend(row.tags.iter().map(|tag| ("tag".to_string(), tag.clone())));
    fields.extend(
        row.metadata
            .iter()
            .map(|(key, value)| (format!("metadata.{key}"), value.clone())),
    );
    fields.extend(
        row.source_urls
            .iter()
            .map(|url| ("source_url".to_string(), url.clone())),
    );
    fields
}

pub(crate) fn registry_web_snippet_matches(value: &str, terms: &[String]) -> bool {
    if terms.is_empty() {
        return true;
    }
    let lower = value.to_lowercase();
    terms.iter().any(|term| lower.contains(term))
}

pub(crate) fn source_preview_start(
    lines: &[&str],
    line_start: Option<usize>,
    line_count: usize,
    terms: &[String],
) -> (usize, Option<usize>) {
    if let Some(line_start) = line_start {
        return (line_start.max(1), None);
    }
    let Some(match_index) = lines
        .iter()
        .position(|line| source_line_matches_terms(line, terms))
    else {
        return (1, None);
    };
    let matched_line = match_index + 1;
    let context = line_count.saturating_div(2);
    (
        matched_line.saturating_sub(context).max(1),
        Some(matched_line),
    )
}

pub(crate) fn source_line_matches_terms(line: &str, terms: &[String]) -> bool {
    if terms.is_empty() {
        return false;
    }
    let line = line.to_lowercase();
    terms.iter().any(|term| line.contains(term))
}

pub(crate) fn json_outline(text: &str) -> Option<serde_json::Value> {
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

pub(crate) fn resolve_registry_source_url(row: &RegistryIndexRow, source_url: &str) -> String {
    if source_url.starts_with("http://") || source_url.starts_with("https://") {
        return source_url.to_string();
    }
    if let Some(base_url) = row.metadata.get("registry_source_base_url") {
        return format!("{}/{source_url}", base_url.trim_end_matches('/'));
    }
    source_url.to_string()
}

pub(crate) fn expected_dataset_ids_from_inputs(
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
