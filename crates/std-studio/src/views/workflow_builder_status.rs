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
    }
}
