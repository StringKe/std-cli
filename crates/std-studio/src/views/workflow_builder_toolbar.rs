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
    Run,
    Cancel,
    Trace,
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

pub(crate) fn render(
    ui: &mut egui::Ui,
    workflow_goal: &mut String,
    can_cancel: bool,
) -> WorkflowToolbarResponse {
    let mut response = WorkflowToolbarResponse::new();
    ui.horizontal_wrapped(|ui| {
        ui.set_min_height(super::workflow_builder_metrics::BUILDER_TOOLBAR_HEIGHT);
        let goal_response = ui.add_sized(
            super::workflow_builder_metrics::goal_input_size(ui.available_width()),
            egui::TextEdit::singleline(workflow_goal)
                .hint_text(i18n::t("studio.workflow_builder.goal.hint")),
        );
        goal_response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::TextEdit,
                ui.is_enabled(),
                workflow_goal_a11y_label(workflow_goal),
            )
        });
        render_primary_actions(ui, &mut response, can_cancel);
        render_secondary_contract(ui);
    });
    response
}

fn render_primary_actions(
    ui: &mut egui::Ui,
    response: &mut WorkflowToolbarResponse,
    can_cancel: bool,
) {
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
        i18n::t("studio.workflow_builder.flow.run"),
        &input::studio_workflow_test().label(),
        true,
    )
    .clicked()
    {
        response.actions.push(WorkflowToolbarAction::Run);
    }
    if can_cancel
        && toolbar_button(ui, i18n::t("studio.workflow_builder.toolbar.cancel"), false).clicked()
    {
        response.actions.push(WorkflowToolbarAction::Cancel);
    }
    if toolbar_button_with_shortcut(
        ui,
        i18n::t("studio.workflow_builder.flow.trace"),
        &input::studio_workflow_history().label(),
        false,
    )
    .clicked()
    {
        response.actions.push(WorkflowToolbarAction::Trace);
    }
}

fn toolbar_button(ui: &mut egui::Ui, label: &str, emphasized: bool) -> egui::Response {
    toolbar_button_text(
        ui,
        label,
        None,
        emphasized,
        toolbar_button_a11y_label(label, None),
    )
}

fn toolbar_button_with_shortcut(
    ui: &mut egui::Ui,
    label: &str,
    shortcut: &str,
    emphasized: bool,
) -> egui::Response {
    toolbar_button_text(
        ui,
        label,
        Some(shortcut),
        emphasized,
        toolbar_button_a11y_label(label, Some(shortcut)),
    )
}

fn toolbar_button_text(
    ui: &mut egui::Ui,
    label: &str,
    shortcut: Option<&str>,
    emphasized: bool,
    a11y_label: String,
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
    let response = ui.add(
        egui::Button::new(
            egui::RichText::new(text)
                .font(Text::caption())
                .color(Color::fg_primary(&ctx)),
        )
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, stroke))
        .corner_radius(egui::CornerRadius::same(Radius::sm())),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            a11y_label.clone(),
        )
    });
    response
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
    "toolbar=goal-input>plan>save>simulate>run>cancel-when-running>trace>ai>zoom;control=token-toolbar-buttons;primary=plan|run;shortcuts=save|simulate|run|trace;a11y=textbox-goal-value,button-label-shortcut-purpose;focus-default=steps-list;run-opens-bottom-panel;simulate=dry-run;cancel=running-only;trace-opens-execution-history"
}

fn workflow_goal_a11y_label(goal: &str) -> String {
    let value = if goal.trim().is_empty() {
        i18n::t("studio.workflow_builder.goal.empty")
    } else {
        goal.trim()
    };
    i18n::t("studio.workflow_builder.goal.a11y").replace("{value}", value)
}

fn toolbar_button_a11y_label(label: &str, shortcut: Option<&str>) -> String {
    match shortcut {
        Some(shortcut) => i18n::t("studio.workflow_builder.toolbar.button_shortcut.a11y")
            .replace("{label}", label)
            .replace("{shortcut}", shortcut),
        None => i18n::t("studio.workflow_builder.toolbar.button.a11y").replace("{label}", label),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow_builder_toolbar_contract_matches_docs_22_order() {
        assert_eq!(
            toolbar_contract(),
            "toolbar=goal-input>plan>save>simulate>run>cancel-when-running>trace>ai>zoom;control=token-toolbar-buttons;primary=plan|run;shortcuts=save|simulate|run|trace;a11y=textbox-goal-value,button-label-shortcut-purpose;focus-default=steps-list;run-opens-bottom-panel;simulate=dry-run;cancel=running-only;trace-opens-execution-history"
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
        assert!(implementation.contains("WidgetType::TextEdit"));
        assert!(implementation.contains("workflow_goal_a11y_label"));
        assert!(implementation.contains("Color::accent_weak"));
        assert!(implementation.contains("Color::bg_surface_1"));
        assert!(implementation.contains("WidgetType::Button"));
        assert!(implementation.contains("toolbar_button_a11y_label"));
        assert!(!implementation.contains("quiet_button"));
    }

    #[test]
    fn toolbar_button_a11y_labels_include_shortcut_when_available() {
        assert_eq!(
            toolbar_button_a11y_label("Save", Some("Mod+S")),
            i18n::t("studio.workflow_builder.toolbar.button_shortcut.a11y")
                .replace("{label}", "Save")
                .replace("{shortcut}", "Mod+S")
        );
        assert_eq!(
            toolbar_button_a11y_label("Plan", None),
            i18n::t("studio.workflow_builder.toolbar.button.a11y").replace("{label}", "Plan")
        );
    }

    #[test]
    fn workflow_goal_a11y_label_exposes_textbox_value() {
        assert_eq!(
            workflow_goal_a11y_label("ship ui"),
            i18n::t("studio.workflow_builder.goal.a11y").replace("{value}", "ship ui")
        );
        assert_eq!(
            workflow_goal_a11y_label("   "),
            i18n::t("studio.workflow_builder.goal.a11y")
                .replace("{value}", i18n::t("studio.workflow_builder.goal.empty"))
        );
    }
}
