use crate::views::{
    workflow_builder_fields, workflow_builder_flow, workflow_builder_toolbar,
    workflow_builder_trace,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorkflowBuilderInteractionContract {
    pub(crate) shell: &'static str,
    pub(crate) flow: &'static str,
    pub(crate) toolbar: &'static str,
    pub(crate) steps: &'static str,
    pub(crate) properties: &'static str,
    pub(crate) ai_assist: &'static str,
    pub(crate) debug: String,
    pub(crate) bottom_panel: String,
    pub(crate) history: &'static str,
}

impl WorkflowBuilderInteractionContract {
    pub(crate) fn new(
        dry_run: Option<&std_orchestration::WorkflowDryRun>,
        execution: Option<&std_orchestration::WorkflowExecution>,
        bottom_panel: String,
    ) -> Self {
        Self {
            shell: "shell=toolbar>status>steps+properties>trace>ai-assist",
            flow: workflow_builder_flow::flow_contract(),
            toolbar: workflow_builder_toolbar::toolbar_contract(),
            steps: crate::views::workflow_builder_step_visual_contract(),
            properties: workflow_builder_fields::fields_contract(),
            ai_assist: "ai_assist=collapsed-input|suggestions|apply|insert|replace",
            debug: workflow_builder_trace::builder_debug_contract(dry_run, execution),
            bottom_panel,
            history: "history=execution-history-pane|timeline|payload",
        }
    }

    pub(crate) fn pass(&self) -> bool {
        self.shell == "shell=toolbar>status>steps+properties>trace>ai-assist"
            && self.flow == "flow=goal-input|plan|save|simulate|test|trace"
            && self.toolbar.contains("toolbar=goal-input>plan>save")
            && self.steps.contains("row=48")
            && self.steps.contains("focus-default=steps-list")
            && self.steps.contains("keyboard-select")
            && self.steps.contains("a11y=row-index-name-type-selected")
            && self.steps.contains("selected=surface-3+accent-left")
            && self
                .properties
                .contains("inputs=step-name|parameters-json|index")
            && self.ai_assist.contains("apply|insert|replace")
            && self.debug.contains("debug_panel=true")
            && self
                .bottom_panel
                .contains("batch-debug=simulate:open|run:open|planned-run:open")
            && self.bottom_panel.contains("history:open")
            && self.bottom_panel.contains("role=bottom-panel-tabs")
            && self.history == "history=execution-history-pane|timeline|payload"
    }

    pub(crate) fn summary(&self) -> String {
        [
            "builder_interaction=single-workbench-flow",
            self.shell,
            self.flow,
            self.toolbar,
            self.steps,
            self.properties,
            self.ai_assist,
            &self.debug,
            &self.bottom_panel,
            self.history,
        ]
        .join(";")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_interaction_contract_covers_docs22_flow() {
        let contract = WorkflowBuilderInteractionContract::new(None, None, "batch-debug=simulate:open|run:open|planned-run:open|history:open;helper=open;tabs=批量调试|日志|问题|性能;selected=批量调试;role=bottom-panel-tabs".to_string());
        let summary = contract.summary();

        assert!(contract.pass(), "{summary}");
        assert!(summary.contains("builder_interaction=single-workbench-flow"));
        assert!(summary.contains("shell=toolbar>status>steps+properties>trace>ai-assist"));
        assert!(summary.contains("flow=goal-input|plan|save|simulate|test|trace"));
        assert!(summary.contains("focus-default=steps-list"));
        assert!(summary.contains("keyboard-select"));
        assert!(summary.contains("a11y=row-index-name-type-selected"));
        assert!(summary.contains("ai_assist=collapsed-input|suggestions|apply|insert|replace"));
        assert!(summary.contains("history=execution-history-pane|timeline|payload"));
    }
}
