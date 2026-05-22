use crate::{ui, StudioEguiApp};
use eframe::egui;
use std_egui::{i18n, tokens::Space};
use std_orchestration::ExecutionStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct WorkflowBuilderStatus {
    pub planned: bool,
    pub saved: bool,
    pub simulated: bool,
    pub ran: bool,
    pub traced: bool,
}

impl WorkflowBuilderStatus {
    pub(crate) fn from_app(app: &StudioEguiApp) -> Self {
        Self {
            planned: app.app.planned_workflow.is_some(),
            saved: app.workflow_selected_path.is_some(),
            simulated: app.app.workflow_debug.is_some(),
            ran: app.app.last_workflow_execution.is_some(),
            traced: app
                .app
                .last_workflow_execution
                .as_ref()
                .map(|execution| matches!(execution.status, ExecutionStatus::Completed))
                .unwrap_or(false),
        }
    }

    pub(crate) fn pass(self) -> bool {
        self.planned && self.saved && self.simulated && self.ran && self.traced
    }

    pub(crate) fn summary(self) -> String {
        format!(
            "planned={},saved={},simulated={},ran={},traced={}",
            self.planned, self.saved, self.simulated, self.ran, self.traced
        )
    }

    pub(crate) fn next_action(self) -> &'static str {
        if !self.planned {
            "plan"
        } else if !self.saved {
            "save"
        } else if !self.simulated {
            "simulate"
        } else if !self.ran {
            "run"
        } else if !self.traced {
            "trace"
        } else {
            "complete"
        }
    }

    pub(crate) fn bottom_panel_contract(self) -> &'static str {
        if self.simulated || self.ran {
            "batch-debug=simulate-or-run:open"
        } else {
            "batch-debug=simulate-or-run:pending"
        }
    }
}

pub(crate) fn render(ui: &mut egui::Ui, app: &StudioEguiApp) {
    let status = WorkflowBuilderStatus::from_app(app);
    ui::subtle_frame(ui.ctx()).show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            ui::chip(
                ui,
                i18n::t("studio.workflow_builder.flow.title"),
                ui::panel_alt(ui.ctx()),
            );
            step_chip(ui, status_label(status), status.pass());
            step_chip(
                ui,
                i18n::t("studio.workflow_builder.flow.plan"),
                status.planned,
            );
            step_chip(
                ui,
                i18n::t("studio.workflow_builder.flow.save"),
                status.saved,
            );
            step_chip(
                ui,
                i18n::t("studio.workflow_builder.flow.simulate"),
                status.simulated,
            );
            step_chip(ui, i18n::t("studio.workflow_builder.flow.run"), status.ran);
            step_chip(
                ui,
                i18n::t("studio.workflow_builder.flow.trace"),
                status.traced,
            );
        });
        ui.add_space(Space::TWO_XS as f32);
        ui.label(
            egui::RichText::new(status.summary())
                .font(std_egui::tokens::Text::caption())
                .color(ui::muted_text(ui.ctx())),
        );
        ui.label(
            egui::RichText::new(format!(
                "next={},{}",
                status.next_action(),
                status.bottom_panel_contract()
            ))
            .font(std_egui::tokens::Text::caption())
            .color(ui::muted_text(ui.ctx())),
        );
    });
}

fn step_chip(ui: &mut egui::Ui, label: &str, done: bool) {
    let fill = if done {
        ui::ok_bg(ui.ctx())
    } else {
        ui::panel_alt(ui.ctx())
    };
    ui::chip(ui, label, fill);
}

fn status_label(status: WorkflowBuilderStatus) -> &'static str {
    if status.pass() {
        "PASS"
    } else {
        "PENDING"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow_builder_status_requires_full_interaction_chain() {
        let status = WorkflowBuilderStatus {
            planned: true,
            saved: true,
            simulated: true,
            ran: true,
            traced: true,
        };

        assert!(status.pass());
        assert_eq!(
            status.summary(),
            "planned=true,saved=true,simulated=true,ran=true,traced=true"
        );
        assert_eq!(status.next_action(), "complete");
        assert_eq!(
            status.bottom_panel_contract(),
            "batch-debug=simulate-or-run:open"
        );
    }

    #[test]
    fn workflow_builder_status_guides_next_required_action() {
        let pending = WorkflowBuilderStatus {
            planned: false,
            saved: false,
            simulated: false,
            ran: false,
            traced: false,
        };
        let planned = WorkflowBuilderStatus {
            planned: true,
            saved: false,
            simulated: false,
            ran: false,
            traced: false,
        };
        let saved = WorkflowBuilderStatus {
            planned: true,
            saved: true,
            simulated: false,
            ran: false,
            traced: false,
        };
        let simulated = WorkflowBuilderStatus {
            planned: true,
            saved: true,
            simulated: true,
            ran: false,
            traced: false,
        };
        let ran = WorkflowBuilderStatus {
            planned: true,
            saved: true,
            simulated: true,
            ran: true,
            traced: false,
        };

        assert_eq!(pending.next_action(), "plan");
        assert_eq!(planned.next_action(), "save");
        assert_eq!(saved.next_action(), "simulate");
        assert_eq!(simulated.next_action(), "run");
        assert_eq!(ran.next_action(), "trace");
        assert_eq!(
            saved.bottom_panel_contract(),
            "batch-debug=simulate-or-run:pending"
        );
        assert_eq!(
            simulated.bottom_panel_contract(),
            "batch-debug=simulate-or-run:open"
        );
    }
}
