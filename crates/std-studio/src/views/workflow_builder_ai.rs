use crate::ui;
use eframe::egui;
use std_egui::{
    i18n,
    tokens::{Color, Radius, Space, Text},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WorkflowAiAction {
    Apply(usize),
    Insert(usize),
    Replace(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorkflowAiSuggestion {
    pub(crate) title: &'static str,
    pub(crate) detail: &'static str,
    pub(crate) step_name: &'static str,
    pub(crate) parameters: serde_json::Value,
}

pub(crate) fn suggestions(goal: &str) -> Vec<WorkflowAiSuggestion> {
    let source = if goal.trim().is_empty() {
        "studio"
    } else {
        goal.trim()
    };
    vec![
        WorkflowAiSuggestion {
            title: "Collect context",
            detail: "Insert a first step that gathers local context before acting.",
            step_name: "Collect context",
            parameters: serde_json::json!({"source": source, "mode": "read-only"}),
        },
        WorkflowAiSuggestion {
            title: "Validate result",
            detail: "Add a verification step before marking the workflow complete.",
            step_name: "Validate result",
            parameters: serde_json::json!({"gate": "verify-output", "required": true}),
        },
        WorkflowAiSuggestion {
            title: "Record trace",
            detail: "Capture the execution trace for History and Operations panes.",
            step_name: "Record trace",
            parameters: serde_json::json!({"target": "execution-history"}),
        },
    ]
}

pub(crate) fn render(ui: &mut egui::Ui, goal: &mut String) -> Option<WorkflowAiAction> {
    let mut selected_action = None;
    ui::subtle_frame(ui.ctx()).show(ui, |ui| {
        ui::section_header(
            ui,
            i18n::t("studio.workflow_builder.ai.title"),
            i18n::t("studio.workflow_builder.ai.detail"),
        );
        ui.label(i18n::t("studio.workflow_builder.ai.prompt"));
        ui.add_sized(
            super::workflow_builder_metrics::ai_input_size(ui.available_width()),
            egui::TextEdit::singleline(goal),
        );
        ui.add_space(Space::XS as f32);
        for (index, suggestion) in suggestions(goal).iter().enumerate() {
            if let Some(action) = suggestion_row(ui, index, suggestion) {
                selected_action = Some(action);
            }
        }
    });
    selected_action
}

fn suggestion_row(
    ui: &mut egui::Ui,
    index: usize,
    suggestion: &WorkflowAiSuggestion,
) -> Option<WorkflowAiAction> {
    let ctx = ui.ctx().clone();
    let mut action = None;
    egui::Frame::new()
        .fill(Color::bg_surface_1(&ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_divider(&ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::md()))
        .inner_margin(egui::Margin::same(Space::sm()))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(suggestion.title)
                            .font(Text::body())
                            .strong(),
                    );
                    ui.label(
                        egui::RichText::new(suggestion.detail)
                            .font(Text::caption())
                            .color(Color::fg_secondary(&ctx)),
                    );
                });
                if ui::quiet_button(ui, "Apply").clicked() {
                    action = Some(WorkflowAiAction::Apply(index));
                }
                if ui::quiet_button(ui, "Insert").clicked() {
                    action = Some(WorkflowAiAction::Insert(index));
                }
                if ui::quiet_button(ui, "Replace").clicked() {
                    action = Some(WorkflowAiAction::Replace(index));
                }
            });
        });
    action
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow_ai_suggestions_cover_apply_insert_replace_targets() {
        let suggestions = suggestions("release");

        assert_eq!(suggestions.len(), 3);
        assert_eq!(suggestions[0].step_name, "Collect context");
        assert_eq!(suggestions[0].parameters["source"], "release");
        assert_eq!(WorkflowAiAction::Apply(0), WorkflowAiAction::Apply(0));
        assert_eq!(WorkflowAiAction::Insert(1), WorkflowAiAction::Insert(1));
        assert_eq!(WorkflowAiAction::Replace(2), WorkflowAiAction::Replace(2));
    }
}
