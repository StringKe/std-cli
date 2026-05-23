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

    pub(crate) fn flow_rail_contract(self) -> String {
        let states = workflow_flow_steps()
            .into_iter()
            .map(|step| step.state_for(self))
            .collect::<Vec<_>>()
            .join("|");
        format!(
            "flow_rail=plan>save>simulate>run>trace;states={states};next={};surface=token-inline-rail;a11y=number-label-state",
            self.next_action()
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct WorkflowFlowStep {
    key: &'static str,
    label: &'static str,
    done: fn(WorkflowBuilderStatus) -> bool,
}

impl WorkflowFlowStep {
    fn state_for(self, status: WorkflowBuilderStatus) -> &'static str {
        if (self.done)(status) {
            "done"
        } else if status.next_action() == self.key {
            "current"
        } else {
            "pending"
        }
    }
}

pub(crate) fn render(ui: &mut egui::Ui, app: &StudioEguiApp) {
    let status = WorkflowBuilderStatus::from_app(app);
    ui::subtle_frame(ui.ctx()).show(ui, |ui| {
        render_flow_rail(ui, status);
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

fn render_flow_rail(ui: &mut egui::Ui, status: WorkflowBuilderStatus) {
    ui.vertical(|ui| {
        ui.horizontal_wrapped(|ui| {
            ui::chip(
                ui,
                i18n::t("studio.workflow_builder.flow.title"),
                ui::panel_alt(ui.ctx()),
            );
            ui::chip(ui, status_label(status), status_fill(ui, status.pass()));
        });
        ui.add_space(Space::TWO_XS as f32);
        ui.horizontal_wrapped(|ui| {
            for (index, step) in workflow_flow_steps().into_iter().enumerate() {
                step_cell(ui, index + 1, step, status);
            }
        });
    });
}

fn step_cell(
    ui: &mut egui::Ui,
    index: usize,
    step: WorkflowFlowStep,
    status: WorkflowBuilderStatus,
) {
    let state = step.state_for(status);
    let fill = match state {
        "done" => ui::ok_bg(ui.ctx()),
        "current" => ui::selected_bg(ui.ctx()),
        _ => ui::panel_alt(ui.ctx()),
    };
    let text = format!("{index} {} {state}", step.label);
    let response = egui::Frame::new()
        .fill(fill)
        .corner_radius(egui::CornerRadius::same(std_egui::tokens::Radius::SM))
        .inner_margin(egui::Margin::symmetric(Space::XS, Space::TWO_XS))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(&text)
                    .font(std_egui::tokens::Text::caption())
                    .color(ui::strong_text(ui.ctx())),
            );
        })
        .response;
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text.clone())
    });
}

fn status_fill(ui: &egui::Ui, done: bool) -> egui::Color32 {
    if done {
        ui::ok_bg(ui.ctx())
    } else {
        ui::panel_alt(ui.ctx())
    }
}

fn status_label(status: WorkflowBuilderStatus) -> &'static str {
    if status.pass() {
        "PASS"
    } else {
        "PENDING"
    }
}

fn workflow_flow_steps() -> [WorkflowFlowStep; 5] {
    [
        WorkflowFlowStep {
            key: "plan",
            label: "Plan",
            done: |status| status.planned,
        },
        WorkflowFlowStep {
            key: "save",
            label: "Save",
            done: |status| status.saved,
        },
        WorkflowFlowStep {
            key: "simulate",
            label: "Simulate",
            done: |status| status.simulated,
        },
        WorkflowFlowStep {
            key: "run",
            label: "Run",
            done: |status| status.ran,
        },
        WorkflowFlowStep {
            key: "trace",
            label: "Trace",
            done: |status| status.traced,
        },
    ]
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
        assert_eq!(
            status.flow_rail_contract(),
            "flow_rail=plan>save>simulate>run>trace;states=done|done|done|done|done;next=complete;surface=token-inline-rail;a11y=number-label-state"
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
        assert_eq!(
            pending.flow_rail_contract(),
            "flow_rail=plan>save>simulate>run>trace;states=current|pending|pending|pending|pending;next=plan;surface=token-inline-rail;a11y=number-label-state"
        );
        assert_eq!(
            saved.flow_rail_contract(),
            "flow_rail=plan>save>simulate>run>trace;states=done|done|current|pending|pending;next=simulate;surface=token-inline-rail;a11y=number-label-state"
        );
    }
}
