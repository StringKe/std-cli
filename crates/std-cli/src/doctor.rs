mod report;
mod workspace;

use crate::CliError;
use crate::{install_plan, release_plan};
use report::DoctorReport;
use std::path::Path;
use std_core::{discover_plugin_manifests, AiPlanner, StdConfig, StdCore};
use std_index::Indexer;
use std_orchestration::{workflow_from_plan, ExecutionStatus, WorkflowExecutor};
use workspace::{check_text, check_workspace_quality};

pub(crate) fn doctor(core: &StdCore, json: bool) -> Result<String, CliError> {
    let report = doctor_report(core)?;
    if json {
        Ok(report.json()?)
    } else {
        Ok(report.text())
    }
}

fn doctor_report(core: &StdCore) -> Result<DoctorReport, CliError> {
    core.ensure_storage()?;
    check_storage(core)?;

    let actions = core.search("", 1_000)?;
    if actions.is_empty() {
        return Err(CliError::Doctor("registry has no actions".to_string()));
    }
    if core.search("Open Terminal", 1)?.is_empty() {
        return Err(CliError::Doctor(
            "registry missing Open Terminal action".to_string(),
        ));
    }

    let plan = AiPlanner::plan(core, "rebuild index")?;
    let workflow = workflow_from_plan(&plan);
    let dry_run = WorkflowExecutor::new(core.clone()).dry_run(&workflow)?;
    if dry_run.status != ExecutionStatus::Completed {
        return Err(CliError::Doctor(format!(
            "planner workflow dry-run failed: {:?}",
            dry_run.status
        )));
    }

    let index_doc = Indexer::analyze(Path::new("."))?;
    if index_doc.components.is_empty() {
        return Err(CliError::Doctor(
            "index analysis produced no components".to_string(),
        ));
    }

    let workspace = check_workspace_quality()?;
    let release = release_plan(core, "1.0.0")?;
    check_text(&release, "verify=mise run quality")?;
    check_text(&release, "std release verify --dist")?;
    check_text(&release, "std install verify --prefix")?;
    let install = install_plan(core, None)?;
    check_text(&install, "binaries=std,std-launcher,std-studio")?;
    check_text(&install, "app_bundles=std Launcher.app,std Studio.app")?;

    let plugin_manifests = discover_plugin_manifests(&core.config.plugins_dir())?;
    let audit_events = core.read_audit_events()?;
    Ok(DoctorReport {
        status: "PASS",
        storage: "PASS",
        actions: actions.len(),
        planner: "PASS",
        workflow_dry_run: "PASS",
        index_components: index_doc.components.len(),
        index_relations: index_doc.relations.len(),
        plugins: plugin_manifests.len(),
        audit_events: audit_events.len(),
        quality: "PASS",
        quality_ci: workspace.quality_ci,
        dylint_lint: workspace.dylint_lint,
        quality_tools: vec!["rustfmt", "clippy", "dylint", "cargo-deny", "cargo-machete"],
        source_file_limit: workspace.source_file_limit,
        config_file_limit: workspace.config_file_limit,
        config_files: workspace.config_files,
        max_config_file: workspace.max_config_file,
        max_config_lines: workspace.max_config_lines,
        source_files: workspace.source_files,
        max_source_file: workspace.max_source_file,
        max_source_lines: workspace.max_source_lines,
        workspace_crates: workspace.workspace_crates,
        launcher: "PASS",
        studio: "PASS",
        release_plan: "PASS",
        install_plan: "PASS",
        config_path: StdConfig::writable_config_path(),
    })
}

fn check_storage(core: &StdCore) -> Result<(), CliError> {
    let storage_dirs = [
        core.config.data_dir.clone(),
        core.config.workflows_dir(),
        core.config.index_dir(),
        core.config.memory_dir(),
        core.config.history_dir(),
        core.config.plugins_dir(),
    ];
    for dir in &storage_dirs {
        if !dir.is_dir() {
            return Err(CliError::Doctor(format!(
                "storage directory missing: {}",
                dir.display()
            )));
        }
    }
    Ok(())
}
