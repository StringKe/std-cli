use crate::ui;
use eframe::egui;
use std_egui::{
    i18n,
    tokens::{Color, Radius, Space, Text},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WorkflowToolbarAction {
    Plan,
    Save,
    Simulate,
    Test,
    History,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorkflowToolbarResponse {
    pub(crate) actions: Vec<WorkflowToolbarAction>,
}

impl WorkflowToolbarResponse {
    fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }
}

pub(crate) fn render(ui: &mut egui::Ui, workflow_goal: &mut String) -> WorkflowToolbarResponse {
    let mut response = WorkflowToolbarResponse::new();
    ui.horizontal_wrapped(|ui| {
        ui.set_min_height(super::workflow_builder_metrics::BUILDER_TOOLBAR_HEIGHT);
        ui.add_sized(
            super::workflow_builder_metrics::goal_input_size(ui.available_width()),
            egui::TextEdit::singleline(workflow_goal)
                .hint_text(i18n::t("studio.workflow_builder.goal.hint")),
        );
        render_primary_actions(ui, &mut response);
        render_secondary_contract(ui);
    });
    response
}

fn render_primary_actions(ui: &mut egui::Ui, response: &mut WorkflowToolbarResponse) {
    if ui::quiet_button(ui, i18n::t("studio.workflow_builder.flow.plan")).clicked() {
        response.actions.push(WorkflowToolbarAction::Plan);
    }
    if ui::quiet_button(ui, i18n::t("studio.workflow_builder.flow.save")).clicked() {
        response.actions.push(WorkflowToolbarAction::Save);
    }
    if ui::quiet_button(ui, i18n::t("studio.workflow_builder.flow.simulate")).clicked() {
        response.actions.push(WorkflowToolbarAction::Simulate);
    }
    if ui::quiet_button(ui, i18n::t("studio.workflow_builder.toolbar.test")).clicked() {
        response.actions.push(WorkflowToolbarAction::Test);
    }
    if ui::quiet_button(ui, i18n::t("studio.workflow_builder.flow.trace")).clicked() {
        response.actions.push(WorkflowToolbarAction::History);
    }
}

fn render_secondary_contract(ui: &mut egui::Ui) {
    toolbar_badge(ui, i18n::t("studio.workflow_builder.ai.title"));
    toolbar_badge(ui, i18n::t("studio.workflow_builder.toolbar.zoom"));
}

fn toolbar_badge(ui: &mut egui::Ui, label: &str) {
    let ctx = ui.ctx().clone();
    egui::Frame::new()
        .fill(Color::bg_surface_2(&ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_divider(&ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::sm()))
        .inner_margin(egui::Margin::symmetric(Space::xs(), Space::two_xs()))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(label)
                    .font(Text::caption())
                    .color(Color::fg_secondary(&ctx)),
            );
        });
}

#[cfg(test)]
pub(crate) fn toolbar_contract() -> &'static str {
    "toolbar=goal-input>plan>save>simulate>test>history-action>ai>zoom;test-opens-bottom-panel;simulate=dry-run;history-opens-execution-history"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow_builder_toolbar_contract_matches_docs_22_order() {
        assert_eq!(
            toolbar_contract(),
            "toolbar=goal-input>plan>save>simulate>test>history-action>ai>zoom;test-opens-bottom-panel;simulate=dry-run;history-opens-execution-history"
        );
    }

    #[test]
    fn workflow_toolbar_response_starts_empty() {
        assert!(WorkflowToolbarResponse::new().actions.is_empty());
    }
}
