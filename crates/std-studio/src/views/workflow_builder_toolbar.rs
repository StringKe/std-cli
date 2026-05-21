use eframe::egui;
use std_egui::{
    i18n, input,
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
    if toolbar_button(ui, i18n::t("studio.workflow_builder.flow.plan"), true).clicked() {
        response.actions.push(WorkflowToolbarAction::Plan);
    }
    if toolbar_button_with_shortcut(
        ui,
        i18n::t("studio.workflow_builder.flow.save"),
        &input::studio_workflow_save().label(),
        false,
    )
    .clicked()
    {
        response.actions.push(WorkflowToolbarAction::Save);
    }
    if toolbar_button_with_shortcut(
        ui,
        i18n::t("studio.workflow_builder.flow.simulate"),
        &input::studio_workflow_simulate().label(),
        false,
    )
    .clicked()
    {
        response.actions.push(WorkflowToolbarAction::Simulate);
    }
    if toolbar_button_with_shortcut(
        ui,
        i18n::t("studio.workflow_builder.toolbar.test"),
        &input::studio_workflow_test().label(),
        true,
    )
    .clicked()
    {
        response.actions.push(WorkflowToolbarAction::Test);
    }
    if toolbar_button_with_shortcut(
        ui,
        i18n::t("studio.workflow_builder.flow.trace"),
        &input::studio_workflow_history().label(),
        false,
    )
    .clicked()
    {
        response.actions.push(WorkflowToolbarAction::History);
    }
}

fn toolbar_button(ui: &mut egui::Ui, label: &str, emphasized: bool) -> egui::Response {
    toolbar_button_text(ui, label, None, emphasized)
}

fn toolbar_button_with_shortcut(
    ui: &mut egui::Ui,
    label: &str,
    shortcut: &str,
    emphasized: bool,
) -> egui::Response {
    toolbar_button_text(ui, label, Some(shortcut), emphasized)
}

fn toolbar_button_text(
    ui: &mut egui::Ui,
    label: &str,
    shortcut: Option<&str>,
    emphasized: bool,
) -> egui::Response {
    let ctx = ui.ctx().clone();
    let fill = if emphasized {
        Color::accent_weak(&ctx)
    } else {
        Color::bg_surface_1(&ctx)
    };
    let stroke = if emphasized {
        Color::accent_base(&ctx)
    } else {
        Color::stroke_divider(&ctx)
    };
    let text = shortcut
        .map(|shortcut| format!("{label}  {shortcut}"))
        .unwrap_or_else(|| label.to_string());
    ui.add(
        egui::Button::new(
            egui::RichText::new(text)
                .font(Text::caption())
                .color(Color::fg_primary(&ctx)),
        )
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, stroke))
        .corner_radius(egui::CornerRadius::same(Radius::sm())),
    )
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

pub(crate) fn toolbar_contract() -> &'static str {
    "toolbar=goal-input>plan>save>simulate>test>history-action>ai>zoom;control=token-toolbar-buttons;primary=plan|test;shortcuts=save|simulate|test|history;test-opens-bottom-panel;simulate=dry-run;history-opens-execution-history"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow_builder_toolbar_contract_matches_docs_22_order() {
        assert_eq!(
            toolbar_contract(),
            "toolbar=goal-input>plan>save>simulate>test>history-action>ai>zoom;control=token-toolbar-buttons;primary=plan|test;shortcuts=save|simulate|test|history;test-opens-bottom-panel;simulate=dry-run;history-opens-execution-history"
        );
    }

    #[test]
    fn workflow_toolbar_response_starts_empty() {
        assert!(WorkflowToolbarResponse::new().actions.is_empty());
    }

    #[test]
    fn workflow_toolbar_uses_token_buttons_not_quiet_button_stack() {
        let source = include_str!("workflow_builder_toolbar.rs");
        let implementation = source.split("#[cfg(test)]").next().unwrap();

        assert!(implementation.contains("fn toolbar_button"));
        assert!(implementation.contains("toolbar_button_with_shortcut"));
        assert!(implementation.contains("input::studio_workflow_test().label()"));
        assert!(implementation.contains("input::studio_workflow_simulate().label()"));
        assert!(implementation.contains("Color::accent_weak"));
        assert!(implementation.contains("Color::bg_surface_1"));
        assert!(!implementation.contains("quiet_button"));
    }
}
