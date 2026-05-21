use crate::{ui, views::workflow_builder_metrics};
use eframe::egui;
use std_egui::i18n;
use std_orchestration::WorkflowStep;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StepPropertyAction {
    Add,
    Update,
    MoveUp,
    MoveDown,
    Remove,
}

#[derive(Debug, Default)]
pub(crate) struct StepPropertyActions {
    pub(crate) actions: Vec<StepPropertyAction>,
}

impl StepPropertyActions {
    pub(crate) fn add_requested(&self) -> bool {
        self.actions.contains(&StepPropertyAction::Add)
    }

    pub(crate) fn update_requested(&self) -> bool {
        self.actions.contains(&StepPropertyAction::Update)
    }

    pub(crate) fn move_up_requested(&self) -> bool {
        self.actions.contains(&StepPropertyAction::MoveUp)
    }

    pub(crate) fn move_down_requested(&self) -> bool {
        self.actions.contains(&StepPropertyAction::MoveDown)
    }

    pub(crate) fn remove_requested(&self) -> bool {
        self.actions.contains(&StepPropertyAction::Remove)
    }
}

pub(crate) fn render_loaded_step_properties(
    ui: &mut egui::Ui,
    step_name: &mut String,
    parameters: &mut String,
    index: &mut String,
) -> StepPropertyActions {
    render_step_fields(ui, step_name, parameters, index, true)
}

pub(crate) fn render_planned_step_properties(
    ui: &mut egui::Ui,
    step_name: &mut String,
    parameters: &mut String,
    index: &mut String,
) -> StepPropertyActions {
    render_step_fields(ui, step_name, parameters, index, false)
}

pub(crate) fn render_step_identity(ui: &mut egui::Ui, index: usize, step: &WorkflowStep) {
    super::workflow_rows::workflow_summary(
        ui,
        &step.name,
        &format!("{:?}", step.step_type),
        index + 1,
    );
    ui.label(
        egui::RichText::new(step.parameters.to_string())
            .font(std_egui::tokens::Text::caption())
            .color(ui::muted_text(ui.ctx())),
    );
    ui.add_space(std_egui::tokens::Space::XS as f32);
}

fn render_step_fields(
    ui: &mut egui::Ui,
    step_name: &mut String,
    parameters: &mut String,
    index: &mut String,
    allow_add: bool,
) -> StepPropertyActions {
    let mut actions = StepPropertyActions::default();
    ui.label(i18n::t("studio.workflow_builder.step_name"));
    ui.text_edit_singleline(step_name);
    ui.label(i18n::t("studio.workflow_builder.parameters"));
    ui.add_sized(
        workflow_builder_metrics::parameter_editor_size(ui.available_width()),
        egui::TextEdit::multiline(parameters),
    );
    ui.horizontal(|ui| {
        ui.label(i18n::t("studio.workflow_builder.index"));
        ui.add_sized(
            workflow_builder_metrics::step_index_size(),
            egui::TextEdit::singleline(index),
        );
        if allow_add && ui::quiet_button(ui, i18n::t("studio.workflow_builder.add")).clicked() {
            actions.actions.push(StepPropertyAction::Add);
        }
        if ui::quiet_button(ui, i18n::t("studio.workflow_builder.update")).clicked() {
            actions.actions.push(StepPropertyAction::Update);
        }
    });
    ui.horizontal_wrapped(|ui| {
        if ui::quiet_button(ui, i18n::t("studio.workflow_builder.move_up")).clicked() {
            actions.actions.push(StepPropertyAction::MoveUp);
        }
        if ui::quiet_button(ui, i18n::t("studio.workflow_builder.move_down")).clicked() {
            actions.actions.push(StepPropertyAction::MoveDown);
        }
        if ui::quiet_button(ui, i18n::t("studio.workflow_builder.remove")).clicked() {
            actions.actions.push(StepPropertyAction::Remove);
        }
    });
    actions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_flags_report_requested_step_operations() {
        let actions = StepPropertyActions {
            actions: vec![
                StepPropertyAction::Add,
                StepPropertyAction::Update,
                StepPropertyAction::MoveDown,
            ],
        };

        assert!(actions.add_requested());
        assert!(actions.update_requested());
        assert!(!actions.move_up_requested());
        assert!(actions.move_down_requested());
        assert!(!actions.remove_requested());
    }
}
