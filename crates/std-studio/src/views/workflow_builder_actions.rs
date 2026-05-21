use crate::{views::workflow_builder_properties::StepPropertyActions, StudioEguiApp};
use std::path::Path;

impl StudioEguiApp {
    pub(crate) fn apply_loaded_step_actions(&mut self, path: &Path, actions: StepPropertyActions) {
        if actions.add_requested() {
            self.add_step_to_selected(path);
        }
        if actions.update_requested() {
            self.update_selected_step(path);
        }
        if actions.move_up_requested() {
            self.move_selected_step(path, -1);
        }
        if actions.move_down_requested() {
            self.move_selected_step(path, 1);
        }
        if actions.remove_requested() {
            self.remove_selected_step(path);
        }
    }

    pub(crate) fn apply_planned_step_actions(&mut self, actions: StepPropertyActions) {
        if actions.update_requested() {
            self.update_planned_step();
        }
        if actions.move_up_requested() {
            self.move_planned_step(-1);
        }
        if actions.move_down_requested() {
            self.move_planned_step(1);
        }
        if actions.remove_requested() {
            self.remove_planned_step();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::views::workflow_builder_properties::StepPropertyAction;

    #[test]
    fn action_dispatch_preserves_loaded_step_order() {
        let actions = StepPropertyActions {
            actions: vec![
                StepPropertyAction::Add,
                StepPropertyAction::Update,
                StepPropertyAction::MoveUp,
                StepPropertyAction::MoveDown,
                StepPropertyAction::Remove,
            ],
        };

        assert!(actions.add_requested());
        assert!(actions.update_requested());
        assert!(actions.move_up_requested());
        assert!(actions.move_down_requested());
        assert!(actions.remove_requested());
    }
}
