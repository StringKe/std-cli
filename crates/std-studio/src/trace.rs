pub fn recent_workflow_traces(
    core: &std_core::StdCore,
    limit: usize,
) -> Result<Vec<std_orchestration::WorkflowExecutionTrace>, TraceError> {
    Ok(std_orchestration::recent_workflow_traces(core, limit)?)
}

#[derive(Debug)]
pub enum TraceError {
    Orchestration(std_orchestration::OrchestrationError),
}

impl std::fmt::Display for TraceError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Orchestration(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for TraceError {}

impl From<std_orchestration::OrchestrationError> for TraceError {
    fn from(error: std_orchestration::OrchestrationError) -> Self {
        Self::Orchestration(error)
    }
}
