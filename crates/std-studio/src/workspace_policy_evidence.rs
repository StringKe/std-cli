use crate::{operations_rows, ui};
use eframe::egui;
use std_egui::{i18n, tokens::Space};
use std_studio::StudioWorkspacePolicy;

pub(crate) fn render(ui: &mut egui::Ui, policy: StudioWorkspacePolicy) {
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
}
