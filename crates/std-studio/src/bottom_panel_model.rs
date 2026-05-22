use std_egui::i18n;
use std_orchestration::ExecutionStatus;
use std_types::ActionExecutionStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BottomPanelTab {
    BatchDebug,
    Logs,
    Problems,
    Performance,
}

impl BottomPanelTab {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::BatchDebug => i18n::t("studio.shell.bottom.batch_debug"),
            Self::Logs => i18n::t("studio.shell.bottom.logs"),
            Self::Problems => i18n::t("studio.shell.bottom.problems"),
            Self::Performance => i18n::t("studio.shell.bottom.performance"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BottomPanelTabModel {
    pub(crate) tabs: Vec<BottomPanelTab>,
    pub(crate) selected: BottomPanelTab,
}

impl BottomPanelTabModel {
    pub(crate) fn for_selected(selected: BottomPanelTab) -> Self {
        Self {
            tabs: vec![
                BottomPanelTab::BatchDebug,
                BottomPanelTab::Logs,
                BottomPanelTab::Problems,
                BottomPanelTab::Performance,
            ],
            selected,
        }
    }

    pub(crate) fn docs22_default() -> Self {
        Self::for_selected(BottomPanelTab::BatchDebug)
    }

    pub(crate) fn labels(&self) -> Vec<&'static str> {
        self.tabs.iter().map(|tab| tab.label()).collect()
    }

    pub(crate) fn contract(&self) -> String {
        format!(
            "tabs={};selected={};role=bottom-panel-tabs",
            self.labels().join("|"),
            self.selected.label()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BottomPanelSnapshot {
    pub title: String,
    pub status: String,
    pub rows: Vec<BottomPanelRow>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BottomPanelRow {
    pub name: String,
    pub status: String,
    pub detail: String,
}

pub(crate) fn workflow_status_label(status: &ExecutionStatus) -> &'static str {
    match status {
        ExecutionStatus::Pending => "pending",
        ExecutionStatus::Running => "running",
        ExecutionStatus::Completed => "success",
        ExecutionStatus::Failed => "error",
        ExecutionStatus::Cancelled => "skipped",
    }
}

pub(crate) fn action_status_label(status: &ActionExecutionStatus) -> &'static str {
    match status {
        ActionExecutionStatus::Completed => "success",
        ActionExecutionStatus::Failed => "error",
        ActionExecutionStatus::NeedsExternalRunner => "skipped",
    }
}

pub(crate) fn bottom_panel_row_a11y_label(row: &BottomPanelRow) -> String {
    format!("{}, status {}, {}", row.name, row.status, row.detail)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bottom_panel_tabs_match_docs22_default_order() {
        let model = BottomPanelTabModel::docs22_default();

        assert_eq!(model.labels(), vec!["批量调试", "日志", "问题", "性能"]);
        assert_eq!(model.selected, BottomPanelTab::BatchDebug);
        assert_eq!(
            model.contract(),
            "tabs=批量调试|日志|问题|性能;selected=批量调试;role=bottom-panel-tabs"
        );
    }

    #[test]
    fn workflow_status_labels_match_docs22_batch_debug_states() {
        assert_eq!(workflow_status_label(&ExecutionStatus::Pending), "pending");
        assert_eq!(workflow_status_label(&ExecutionStatus::Running), "running");
        assert_eq!(
            workflow_status_label(&ExecutionStatus::Completed),
            "success"
        );
        assert_eq!(workflow_status_label(&ExecutionStatus::Failed), "error");
        assert_eq!(
            workflow_status_label(&ExecutionStatus::Cancelled),
            "skipped"
        );
    }

    #[test]
    fn action_status_labels_reuse_docs22_batch_debug_states() {
        assert_eq!(
            action_status_label(&ActionExecutionStatus::Completed),
            "success"
        );
        assert_eq!(action_status_label(&ActionExecutionStatus::Failed), "error");
        assert_eq!(
            action_status_label(&ActionExecutionStatus::NeedsExternalRunner),
            "skipped"
        );
    }

    #[test]
    fn bottom_panel_rows_expose_status_and_detail_to_accessibility() {
        let row = BottomPanelRow {
            name: "Run tests".to_string(),
            status: "success".to_string(),
            detail: "started=1 finished=2".to_string(),
        };

        assert_eq!(
            bottom_panel_row_a11y_label(&row),
            "Run tests, status success, started=1 finished=2"
        );
    }
}
