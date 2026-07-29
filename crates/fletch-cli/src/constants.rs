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


pub(crate) const REGISTRY_WEB_HTML: &str = r#"<!doctype html>
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
    .meta, .tags, .sources, .snippet { color: #94a3b8; font-size: .9rem; overflow-wrap: anywhere; }
    .snippet { color: #dbeafe; margin-top: .35rem; }
    mark { background: #fde68a; color: #111827; border-radius: .2rem; padding: 0 .1rem; }
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
        <h3>Presets</h3>
        <div id="presets" class="card meta">Loading presets...</div>
        <div id="facets" class="card meta">Loading facets...</div>
      </nav>
      <section>
        <div id="result-count" class="meta"></div>
        <div class="pager">
          <button type="button" id="prev-page">Previous page</button>
          <label class="meta">Sort
            <select id="sort">
              <option value="relevance" selected>Relevance</option>
              <option value="index">Index order</option>
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
              <option value="desc" selected>Desc</option>
              <option value="asc">Asc</option>
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
          <button type="button" id="copy-link">Copy link</button>
          <button type="button" id="export-csv">Export CSV</button>
          <button type="button" id="export-all-csv">Export all CSV</button>
          <button type="button" id="export-json">Export JSON</button>
          <button type="button" id="export-all-json">Export all JSON</button>
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
    const presets = document.querySelector('#presets');
    const prevPage = document.querySelector('#prev-page');
    const nextPage = document.querySelector('#next-page');
    const pageSize = document.querySelector('#page-size');
    const sort = document.querySelector('#sort');
    const direction = document.querySelector('#direction');
    const copyLink = document.querySelector('#copy-link');
    const exportCsv = document.querySelector('#export-csv');
    const exportAllCsv = document.querySelector('#export-all-csv');
    const exportJson = document.querySelector('#export-json');
    const exportAllJson = document.querySelector('#export-all-json');
    let currentOffset = 0;
    let matchedRowCount = 0;

    function esc(value) {
      return String(value ?? '').replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
    }

    function rowCard(row, snippet, score) {
      const div = document.createElement('div');
      div.className = 'card row';
      div.innerHTML = `<strong>${esc(row.fletch_id)}</strong>
        <div class="meta">${esc(row.registry_id)} · ${esc(row.node_kind)} · score ${esc(score ?? 0)}</div>
        <div class="snippet">${highlightSnippet(snippet, row.fletch_id)}</div>
        <div class="tags">${(row.tags || []).map(tag => `<span class="tag">${esc(tag)}</span>`).join('')}</div>
        <div class="sources">${(row.source_urls || []).map(url => `<div>${esc(url)}</div>`).join('')}</div>`;
      div.addEventListener('click', () => showRowDetail(row, true));
      return div;
    }

    function highlightSnippet(snippet, fallback) {
      const text = String(snippet?.text || fallback || '');
      const terms = [...new Set(snippet?.matched_terms || [])].filter(Boolean).sort((a, b) => b.length - a.length);
      return highlightText(text, terms);
    }

    function highlightText(text, terms) {
      const uniqueTerms = [...new Set(terms || [])].filter(Boolean).sort((a, b) => b.length - a.length);
      if (!uniqueTerms.length) return esc(text);
      const pattern = new RegExp(`(${uniqueTerms.map(escapeRegExp).join('|')})`, 'ig');
      return esc(text).replace(pattern, '<mark>$1</mark>');
    }

    function escapeRegExp(value) {
      return String(value).replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    }

    function showDetail(row) {
      showRowDetail(row, false, true);
    }

    function showRowDetail(row, updateUrl, autoLoadFirstSource) {
      if (updateUrl) updateSelectedRowUrl(row);
      const urls = (row.source_urls || []).map((url, index) => {
        const link = url.startsWith('http') ? `<a href="${esc(url)}" target="_blank" rel="noreferrer">${esc(url)}</a>` : esc(url);
        return `<li>${link} <button type="button" onclick="loadSource('${esc(row.registry_id)}','${esc(row.fletch_id)}',${index})">Load preview</button></li>`;
      }).join('');
      detail.dataset.registryId = row.registry_id;
      detail.dataset.fletchId = row.fletch_id;
      detail.dataset.sourceIndex = '0';
      detail.innerHTML = `<h3>${esc(row.fletch_id)}</h3>
        <button type="button" onclick="copySelectedRowLink()">Copy row link</button>
        <div class="meta">${esc(row.registry_id)} · ${esc(row.node_kind)}</div>
        <h4>Sources</h4><ul>${urls}</ul>
        <h4>Tags</h4><div>${(row.tags || []).map(tag => `<span class="tag">${esc(tag)}</span>`).join('')}</div>
        <h4>Metadata</h4><pre>${esc(JSON.stringify(row.metadata || {}, null, 2))}</pre>
        <h4>Loaded source preview</h4>
        <div id="source-controls" class="meta"></div>
        <pre id="source-preview">Click "Load preview" beside a source URL to fetch bounded source data.</pre>`;
      if (autoLoadFirstSource) loadFirstSourcePreview(row);
    }

    function updateSelectedRowUrl(row) {
      const next = new URL(window.location.href);
      next.searchParams.set('selected_registry_id', row.registry_id);
      next.searchParams.set('selected_fletch_id', row.fletch_id);
      next.searchParams.delete('selected_source');
      next.searchParams.delete('selected_line_start');
      next.searchParams.delete('selected_matched_only');
      window.history.replaceState({}, '', next);
    }

    function selectedRegistryFromUrl() {
      const params = new URLSearchParams(window.location.search);
      const registryId = params.get('selected_registry_id');
      const fletchId = params.get('selected_fletch_id');
      const sourceIndex = params.get('selected_source');
      const lineStart = params.get('selected_line_start');
      const matchedOnly = params.get('selected_matched_only');
      return registryId && fletchId ? {
        registryId,
        fletchId,
        sourceIndex: sourceIndex === null ? null : Number(sourceIndex),
        lineStart: lineStart === null ? null : Number(lineStart),
        matchedOnly: matchedOnly === 'true'
      } : null;
    }

    async function loadSelectedRowDetail(selected) {
      const params = new URLSearchParams({ registry_id: selected.registryId, fletch_id: selected.fletchId });
      const response = await fetch(`/api/row?${params}`);
      if (response.ok) {
        const row = await response.json();
        showRowDetail(row, false, !hasSelectedSourcePreview(selected));
        if (hasSelectedSourcePreview(selected)) loadSelectedSourcePreview(selected);
      }
    }

    function copySelectedRowLink() {
      if (navigator.clipboard) navigator.clipboard.writeText(window.location.href);
    }

    function loadFirstSourcePreview(row) {
      if ((row.source_urls || []).length) loadSource(row.registry_id, row.fletch_id, 0);
    }

    async function loadSource(registryId, fletchId, sourceIndex, lineStart = null, matchedOnly = false) {
      const preview = document.querySelector('#source-preview');
      const controls = document.querySelector('#source-controls');
      detail.dataset.registryId = registryId;
      detail.dataset.fletchId = fletchId;
      detail.dataset.sourceIndex = String(sourceIndex);
      preview.textContent = 'Loading source preview...';
      const params = new URLSearchParams({ registry_id: registryId, fletch_id: fletchId, source: String(sourceIndex), line_count: '80' });
      const text = document.querySelector('#text').value.trim();
      if (text) params.set('text', text);
      if (lineStart !== null) params.set('line_start', String(lineStart));
      if (matchedOnly) params.set('matched_only', 'true');
      const response = await fetch(`/api/source?${params}`);
      if (!response.ok) {
        preview.textContent = `Could not load source preview: ${response.status} ${response.statusText}`;
        return;
      }
      const data = await response.json();
      updateSelectedSourceUrl(registryId, fletchId, sourceIndex, data.line_start, data.matched_only);
      const next = data.line_start + data.line_count;
      const prev = Math.max(1, data.line_start - 80);
      const outline = data.json_outline ? `\nJSON outline: ${JSON.stringify(data.json_outline)}` : '';
      const match = data.matched_line ? ` · First match line: ${data.matched_line}` : '';
      const matchCount = data.matched_terms?.length ? ` · Matched preview lines: ${data.matched_line_count}` : '';
      const matchedOnlyButton = data.matched_terms?.length ? '<button type="button" onclick="loadMatchedCurrentSource()">Matched lines only</button>' : '';
      controls.innerHTML = `Resolved: <a href="${esc(data.resolved_url)}" target="_blank" rel="noreferrer">${esc(data.resolved_url)}</a><br>
        Bytes: ${data.byte_count}${data.truncated ? ' (truncated)' : ''} · Lines: ${data.total_line_count}${match}${matchCount}${outline}<br>
        <button type="button" onclick="loadCurrentSource(${prev})">Previous lines</button>
        <button type="button" onclick="loadCurrentSource(${next})">Next lines</button>
        <button type="button" onclick="copySelectedSourceLink()">Copy source link</button>
        ${matchedOnlyButton}`;
      const terms = currentTextSearchTerms();
      preview.innerHTML = data.lines
        .map(line => `${esc(String(line.number).padStart(5, ' '))}  ${highlightText(line.text, terms)}`)
        .join('\n');
    }

    function updateSelectedSourceUrl(registryId, fletchId, sourceIndex, lineStart, matchedOnly) {
      const next = new URL(window.location.href);
      next.searchParams.set('selected_registry_id', registryId);
      next.searchParams.set('selected_fletch_id', fletchId);
      next.searchParams.set('selected_source', String(sourceIndex));
      next.searchParams.set('selected_line_start', String(lineStart));
      if (matchedOnly) {
        next.searchParams.set('selected_matched_only', 'true');
      } else {
        next.searchParams.delete('selected_matched_only');
      }
      window.history.replaceState({}, '', next);
    }

    function loadSelectedSourcePreview(selected) {
      if (!hasSelectedSourcePreview(selected)) return;
      loadSource(selected.registryId, selected.fletchId, selected.sourceIndex, selected.lineStart, selected.matchedOnly);
    }

    function hasSelectedSourcePreview(selected) {
      return selected?.sourceIndex !== null && !Number.isNaN(selected?.sourceIndex);
    }

    function loadCurrentSource(lineStart) {
      loadSource(detail.dataset.registryId, detail.dataset.fletchId, Number(detail.dataset.sourceIndex || '0'), lineStart);
    }

    function loadMatchedCurrentSource() {
      loadSource(detail.dataset.registryId, detail.dataset.fletchId, Number(detail.dataset.sourceIndex || '0'), null, true);
    }

    function copySelectedSourceLink() {
      if (navigator.clipboard) navigator.clipboard.writeText(window.location.href);
    }

    function currentTextSearchTerms() {
      const text = document.querySelector('#text').value.trim();
      return text ? [text, ...text.split(/\s+/)].filter(Boolean) : [];
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

    function applyPreset(preset) {
      document.querySelector('#text').value = preset.text || '';
      document.querySelector('#tag').value = preset.tag || '';
      document.querySelector('#metadata').value = preset.metadata || '';
      sort.value = preset.sort || 'relevance';
      direction.value = preset.direction || 'desc';
      runSearch(undefined, 0);
    }

    function presetChips(items) {
      return (items || []).map((preset, index) => {
        window.registryWebPresets[index] = preset;
        return `<span class="facet" onclick="applyPreset(window.registryWebPresets[${index}])">${esc(preset.label)}</span>`;
      }).join('') || '<div class="meta">No presets</div>';
    }

    function setControlFromQuery(params, key, selector) {
      const value = params.get(key);
      if (value !== null) {
        document.querySelector(selector).value = value;
      }
    }

    function loadControlsFromUrl() {
      const params = new URLSearchParams(window.location.search);
      setControlFromQuery(params, 'text', '#text');
      setControlFromQuery(params, 'tag', '#tag');
      setControlFromQuery(params, 'metadata', '#metadata');
      setControlFromQuery(params, 'limit', '#page-size');
      setControlFromQuery(params, 'sort', '#sort');
      setControlFromQuery(params, 'direction', '#direction');
      return Number(params.get('offset') || '0');
    }

    function updateBrowserUrl(params, pushState) {
      const next = new URL(window.location.href);
      next.search = params.toString();
      if (pushState) {
        window.history.pushState({}, '', next);
      } else {
        window.history.replaceState({}, '', next);
      }
    }

    function copyShareLink() {
      if (!navigator.clipboard) {
        copyLink.textContent = 'Copy unavailable';
        setTimeout(() => { copyLink.textContent = 'Copy link'; }, 1200);
        return;
      }
      navigator.clipboard.writeText(window.location.href).then(() => {
        copyLink.textContent = 'Copied';
        setTimeout(() => { copyLink.textContent = 'Copy link'; }, 1200);
      }, () => {
        copyLink.textContent = 'Copy failed';
        setTimeout(() => { copyLink.textContent = 'Copy link'; }, 1200);
      });
    }

    function currentSearchParams(offset, selected) {
      const params = new URLSearchParams();
      const text = document.querySelector('#text').value.trim();
      const tag = document.querySelector('#tag').value.trim();
      const metadata = document.querySelector('#metadata').value.trim();
      if (text) params.set('text', text);
      if (tag) tag.split(',').map(v => v.trim()).filter(Boolean).forEach(v => params.append('tag', v));
      if (metadata) metadata.split(',').map(v => v.trim()).filter(Boolean).forEach(v => params.append('metadata', v));
      params.set('offset', String(offset));
      params.set('limit', pageSize.value);
      params.set('sort', sort.value);
      params.set('direction', direction.value);
      if (selected) {
        params.set('selected_registry_id', selected.registryId);
        params.set('selected_fletch_id', selected.fletchId);
        if (selected.sourceIndex !== null && !Number.isNaN(selected.sourceIndex)) {
          params.set('selected_source', String(selected.sourceIndex));
        }
        if (selected.lineStart !== null && !Number.isNaN(selected.lineStart)) {
          params.set('selected_line_start', String(selected.lineStart));
        }
        if (selected.matchedOnly) {
          params.set('selected_matched_only', 'true');
        }
      }
      return params;
    }

    function exportCurrentCsv() {
      window.open(`/api/export.csv?${currentSearchParams(currentOffset)}`, '_blank', 'noreferrer');
    }

    function exportAllCsvMatches() {
      const params = currentSearchParams(0);
      params.set('all', 'true');
      window.open(`/api/export.csv?${params}`, '_blank', 'noreferrer');
    }

    function exportCurrentJson() {
      window.open(`/api/export.json?${currentSearchParams(currentOffset)}`, '_blank', 'noreferrer');
    }

    function exportAllJsonMatches() {
      const params = currentSearchParams(0);
      params.set('all', 'true');
      window.open(`/api/export.json?${params}`, '_blank', 'noreferrer');
    }

    async function runSearch(event, offset = 0, pushState = true, preserveSelection = false) {
      event?.preventDefault();
      currentOffset = Math.max(0, offset);
      const selected = preserveSelection ? selectedRegistryFromUrl() : null;
      const params = currentSearchParams(currentOffset, selected);
      updateBrowserUrl(params, pushState);
      const response = await fetch(`/api/search?${params}`);
      const report = await response.json();
      matchedRowCount = report.matched_row_count;
      const first = matchedRowCount === 0 ? 0 : currentOffset + 1;
      const last = currentOffset + report.rows.length;
      count.textContent = `${matchedRowCount} matches (${first}-${last} shown)`;
      prevPage.disabled = currentOffset === 0;
      nextPage.disabled = currentOffset + report.rows.length >= matchedRowCount;
      results.replaceChildren(...report.rows.map((row, index) => rowCard(row, report.snippets?.[index], report.scores?.[index])));
      if (selected) {
        const selectedRow = report.rows.find(row => row.registry_id === selected.registryId && row.fletch_id === selected.fletchId);
        if (selectedRow) {
          showRowDetail(selectedRow, false, !hasSelectedSourcePreview(selected));
          if (hasSelectedSourcePreview(selected)) loadSelectedSourcePreview(selected);
        } else {
          await loadSelectedRowDetail(selected);
        }
      } else if (report.rows[0]) {
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
    window.registryWebPresets = [];
    fetch('/api/presets').then(r => r.json()).then(data => {
      presets.innerHTML = presetChips(data);
    });
    document.querySelector('#search').addEventListener('submit', event => runSearch(event, 0));
    prevPage.addEventListener('click', () => runSearch(undefined, currentOffset - Number(pageSize.value || '50')));
    nextPage.addEventListener('click', () => runSearch(undefined, currentOffset + Number(pageSize.value || '50')));
    pageSize.addEventListener('change', () => runSearch(undefined, 0));
    sort.addEventListener('change', () => runSearch(undefined, 0));
    direction.addEventListener('change', () => runSearch(undefined, 0));
    copyLink.addEventListener('click', copyShareLink);
    exportCsv.addEventListener('click', exportCurrentCsv);
    exportAllCsv.addEventListener('click', exportAllCsvMatches);
    exportJson.addEventListener('click', exportCurrentJson);
    exportAllJson.addEventListener('click', exportAllJsonMatches);
    window.addEventListener('popstate', () => runSearch(undefined, loadControlsFromUrl(), false, true));
    runSearch(undefined, loadControlsFromUrl(), false, true);
  </script>
</body>
</html>
"#;
