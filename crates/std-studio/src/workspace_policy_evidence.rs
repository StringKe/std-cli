use crate::{operations_rows, ui};
use eframe::egui;
use std_egui::{i18n, tokens::Space};
use std_studio::{StudioApp, StudioWorkspacePolicy};

pub(crate) struct WorkspacePolicyState {
    pub(crate) open_panes: usize,
    pub(crate) focused_pane: String,
    pub(crate) focused_key: String,
    pub(crate) restore_policy: &'static str,
}

impl WorkspacePolicyState {
    pub(crate) fn from_app(app: &StudioApp) -> Self {
        let focused = app
            .focused_pane
            .and_then(|focused| app.open_workspace_panes().find(|pane| pane.id == focused));
        Self {
            open_panes: app.open_workspace_panes().count(),
            focused_pane: focused
                .map(|pane| pane.title.clone())
                .unwrap_or_else(|| "none".to_string()),
            focused_key: focused
                .map(|pane| pane.kind.content_key().to_string())
                .unwrap_or_else(|| "none".to_string()),
            restore_policy: "closeguard=internal-pane-state-only",
        }
    }
}

pub(crate) fn render_with_state(ui: &mut egui::Ui, app: &StudioApp) {
    render_policy(
        ui,
        app.workspace_policy,
        Some(WorkspacePolicyState::from_app(app)),
    );
}

fn render_policy(
    ui: &mut egui::Ui,
    policy: StudioWorkspacePolicy,
    state: Option<WorkspacePolicyState>,
) {
    ui::surface_frame(ui.ctx()).show(ui, |ui| {
        ui::section_header(
            ui,
            i18n::t("studio.operations.workspace_policy.title"),
            i18n::t("studio.operations.workspace_policy.detail"),
        );
        ui.horizontal_wrapped(|ui| {
            ui::chip(ui, policy.summary(), ui::panel_alt(ui.ctx()));
            ui::chip(
                ui,
                policy_state_label(policy),
                policy_state_fill(ui.ctx(), policy),
            );
        });
        ui.add_space(Space::XS as f32);
        operations_rows::gate_row(
            ui,
            i18n::t("studio.operations.workspace_policy.host"),
            "single-borderless-egui-viewport",
            i18n::t("studio.operations.workspace_policy.host.detail"),
        );
        operations_rows::gate_row(
            ui,
            i18n::t("studio.operations.workspace_policy.panes"),
            "internal-egui-workspace-panes",
            i18n::t("studio.operations.workspace_policy.panes.detail"),
        );
        operations_rows::gate_row(
            ui,
            i18n::t("studio.operations.workspace_policy.native"),
            bool_label(policy.allows_native_child_windows()),
            i18n::t("studio.operations.workspace_policy.native.detail"),
        );
        operations_rows::gate_row(
            ui,
            i18n::t("studio.operations.workspace_policy.detached"),
            bool_label(policy.allows_detached_panels()),
            i18n::t("studio.operations.workspace_policy.detached.detail"),
        );
        operations_rows::gate_row(
            ui,
            i18n::t("studio.operations.workspace_policy.docs"),
            StudioWorkspacePolicy::DOC_REFERENCE,
            i18n::t("studio.operations.workspace_policy.docs.detail"),
        );
        operations_rows::gate_row(
            ui,
            i18n::t("studio.operations.workspace_policy.ui_completion"),
            StudioWorkspacePolicy::UI_COMPLETION_BOUNDARY,
            i18n::t("studio.operations.workspace_policy.ui_completion.detail"),
        );
        if let Some(state) = state {
            operations_rows::gate_row(
                ui,
                i18n::t("studio.operations.workspace_policy.open_panes"),
                &state.open_panes.to_string(),
                i18n::t("studio.operations.workspace_policy.open_panes.detail"),
            );
            operations_rows::gate_row(
                ui,
                i18n::t("studio.operations.workspace_policy.focused"),
                &state.focused_pane,
                &state.focused_key,
            );
            operations_rows::gate_row(
                ui,
                i18n::t("studio.operations.workspace_policy.restore"),
                state.restore_policy,
                i18n::t("studio.operations.workspace_policy.restore.detail"),
            );
        }
        operations_rows::gate_row(
            ui,
            i18n::t("studio.operations.workspace_policy.manual_gates"),
            &StudioWorkspacePolicy::MANUAL_UI_EVIDENCE_GATES.join("|"),
            i18n::t("studio.operations.workspace_policy.manual_gates.detail"),
        );
    });
}

fn policy_state_label(policy: StudioWorkspacePolicy) -> &'static str {
    if policy.allows_native_child_windows() || policy.allows_detached_panels() {
        "FAIL"
    } else {
        "PASS"
    }
}

fn policy_state_fill(ctx: &egui::Context, policy: StudioWorkspacePolicy) -> egui::Color32 {
    if policy_state_label(policy) == "PASS" {
        ui::ok_bg(ctx)
    } else {
        ui::warn_bg(ctx)
    }
}

fn bool_label(value: bool) -> &'static str {
    if value {
        "true"
    } else {
        "false"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std_studio::StudioPane;

    #[test]
    fn workspace_policy_evidence_marks_v1_policy_pass() {
        let policy = StudioWorkspacePolicy::studio_v1();

        assert_eq!(policy_state_label(policy), "PASS");
        assert_eq!(bool_label(policy.allows_native_child_windows()), "false");
        assert_eq!(bool_label(policy.allows_detached_panels()), "false");
        assert_eq!(
            StudioWorkspacePolicy::UI_COMPLETION_BOUNDARY,
            "headless-smoke-is-not-ui-completion"
        );
        assert!(StudioWorkspacePolicy::MANUAL_UI_EVIDENCE_GATES.contains(&"light-dark-screenshots"));
    }

    #[test]
    fn workspace_policy_state_reads_current_panes_and_focus() {
        let mut app = StudioApp::default();
        let dashboard = app.open_workspace_pane(StudioPane::Dashboard);
        app.open_plugin_manager_pane();

        assert!(app.focus_workspace_pane(dashboard));

        let state = WorkspacePolicyState::from_app(&app);

        assert_eq!(state.open_panes, 2);
        assert_eq!(state.focused_pane, "Dashboard");
        assert_eq!(state.focused_key, "dashboard");
        assert_eq!(state.restore_policy, "closeguard=internal-pane-state-only");
    }

    #[test]
    fn workspace_policy_operations_surface_uses_runtime_state() {
        let source = include_str!("operations.rs");
        let evidence = include_str!("workspace_policy_evidence.rs");

        assert!(source.contains("workspace_policy_evidence::render_with_state(ui, &self.app)"));
        assert!(evidence.contains("WorkspacePolicyState"));
        assert!(evidence.contains("studio.operations.workspace_policy.open_panes"));
        assert!(evidence.contains("studio.operations.workspace_policy.focused"));
        assert!(evidence.contains("closeguard=internal-pane-state-only"));
    }
}
