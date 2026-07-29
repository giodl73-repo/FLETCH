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

mod constants;
mod cli;
mod types;
mod commands;
mod support;

pub(crate) use constants::*;
pub(crate) use cli::*;
pub(crate) use types::*;
pub(crate) use commands::*;
pub(crate) use support::*;

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
            PublishCommands::MdcropIndex {
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
                let index = mdcrop_index_from_manifest(&manifest, &freshness, max_tip_bytes)?;
                write_json(
                    &slice_mdcrop_index_report(&index, row_type.as_deref(), offset, limit),
                    output,
                )?;
            }
            PublishCommands::ProofDocs {
                mdcrop_index,
                offset,
                limit,
                output,
            } => {
                let mdcrop_index = read_mdcrop_index(&mdcrop_index)?;
                let docs = mdloom_document_manifest(&mdcrop_index);
                write_json(
                    &slice_mdloom_document_manifest(&docs, offset, limit),
                    output,
                )?;
            }
            PublishCommands::LocalUrlMap {
                mdloom_docs,
                base_path,
                offset,
                limit,
                output,
            } => {
                let mdloom_docs = read_mdloom_docs(&mdloom_docs)?;
                let urls = local_url_map(&mdloom_docs, base_path);
                write_json(&slice_local_url_map(&urls, offset, limit), output)?;
            }
            PublishCommands::Bundle {
                mdcrop_index,
                mdloom_docs,
                local_url_map,
                quiver_summary,
                adapter_handoff,
                output,
            } => {
                let mdcrop_index = read_mdcrop_index(&mdcrop_index)?;
                let mdloom_docs = read_mdloom_docs(&mdloom_docs)?;
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
                        &mdcrop_index,
                        &mdloom_docs,
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

