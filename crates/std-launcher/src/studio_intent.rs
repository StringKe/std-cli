use crate::LauncherState;
use std::collections::HashMap;
use std_types::ActionType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudioLaunchIntent {
    pub command: String,
    pub target: StudioLaunchTarget,
    pub source_action: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudioLaunchTarget {
    ExecutionHistory,
    Workflows,
    Memory,
    Plugins,
    Analysis,
    Apps,
}

impl StudioLaunchTarget {
    fn command(self) -> &'static str {
        match self {
            Self::ExecutionHistory => "std-studio --open history",
            Self::Workflows => "std-studio --open workflows",
            Self::Memory => "std-studio --open memory",
            Self::Plugins => "std-studio --open plugins",
            Self::Analysis => "std-studio --open analysis",
            Self::Apps => "std-studio --open apps",
        }
    }
}

impl LauncherState {
    pub fn open_studio_execution_history_from_feedback(&mut self) -> StudioLaunchIntent {
        let source_action = self
            .view
            .feedback
            .as_ref()
            .map(|feedback| feedback.action_name.clone())
            .unwrap_or_else(|| "Launcher feedback".to_string());
        self.record_studio_intent(StudioLaunchTarget::ExecutionHistory, source_action)
    }

    pub fn open_selected_action_in_studio(&mut self) -> Option<StudioLaunchIntent> {
        let result = self.view.selected_result()?.clone();
        let preview = self.core.preview_action(result.action.id).ok();
        let empty_metadata = HashMap::new();
        let metadata = preview
            .as_ref()
            .map(|preview| &preview.metadata)
            .unwrap_or(&empty_metadata);
        let target = studio_target_for_action(&result.action.action_type, metadata);
        Some(self.record_studio_intent(target, result.action.name))
    }

    fn record_studio_intent(
        &mut self,
        target: StudioLaunchTarget,
        source_action: String,
    ) -> StudioLaunchIntent {
        let intent = StudioLaunchIntent {
            command: target.command().to_string(),
            target,
            source_action,
        };
        self.studio_intent = Some(intent.clone());
        intent
    }
}

fn studio_target_for_action(
    action_type: &ActionType,
    metadata: &HashMap<String, String>,
) -> StudioLaunchTarget {
    match action_type {
        ActionType::Workflow => StudioLaunchTarget::Workflows,
        ActionType::Skill | ActionType::Clipboard => StudioLaunchTarget::Memory,
        ActionType::Command if metadata.contains_key("plugin") => StudioLaunchTarget::Plugins,
        ActionType::Command => StudioLaunchTarget::Analysis,
        ActionType::AppLaunch => StudioLaunchTarget::Apps,
        ActionType::Custom(kind) if kind == "file" => StudioLaunchTarget::Analysis,
        ActionType::Custom(_) => StudioLaunchTarget::Analysis,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_launcher_action_types_to_studio_targets() {
        let empty = HashMap::new();
        assert_eq!(
            studio_target_for_action(&ActionType::Workflow, &empty),
            StudioLaunchTarget::Workflows
        );
        assert_eq!(
            studio_target_for_action(&ActionType::Skill, &empty),
            StudioLaunchTarget::Memory
        );
        assert_eq!(
            studio_target_for_action(&ActionType::Command, &empty),
            StudioLaunchTarget::Analysis
        );
        assert_eq!(
            studio_target_for_action(&ActionType::AppLaunch, &empty),
            StudioLaunchTarget::Apps
        );
        let plugin = HashMap::from([("plugin".to_string(), "demo".to_string())]);
        assert_eq!(
            studio_target_for_action(&ActionType::Command, &plugin),
            StudioLaunchTarget::Plugins
        );
    }
}
