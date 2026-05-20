use crate::CliError;
use std::{fs, path::Path};
use std_core::StdCore;
use std_orchestration::{BatchExecutor, BatchPlan};

pub(crate) fn run_batch_file(
    core: &StdCore,
    path: &Path,
    allow_external: bool,
    stop_on_error: bool,
) -> Result<String, CliError> {
    let body = fs::read_to_string(path)?;
    let mut plan: BatchPlan = serde_json::from_str(&body)?;
    plan.allow_external = plan.allow_external || allow_external;
    plan.stop_on_error = plan.stop_on_error || stop_on_error;
    let report = BatchExecutor::new(core.clone()).execute(&plan);
    Ok(serde_json::to_string_pretty(&report)?)
}
