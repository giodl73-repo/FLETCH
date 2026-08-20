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

pub(crate) fn run_browser_launcher(mut command: std::process::Command) -> Result<()> {
    let status = command.status().context("failed to launch browser")?;
    if !status.success() {
        bail!("browser launcher exited with status {status}");
    }
    Ok(())
}
