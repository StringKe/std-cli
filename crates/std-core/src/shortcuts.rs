use crate::StdConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutScope {
    Launcher,
    Studio,
    WorkflowBuilder,
    AnalysisWorkbench,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutSource {
    Default,
    User,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutSpec {
    pub id: &'static str,
    pub scope: ShortcutScope,
    pub action: &'static str,
    pub binding: String,
    pub default_binding: &'static str,
    pub source: ShortcutSource,
    pub resettable: bool,
}

pub fn shortcut_registry(config: &StdConfig) -> Vec<ShortcutSpec> {
    let launcher_binding = config.launcher_hotkey.clone();
    let launcher_source = if launcher_binding == "Alt+Space" {
        ShortcutSource::Default
    } else {
        ShortcutSource::User
    };
    vec![
        ShortcutSpec {
            id: "launcher.global.toggle",
            scope: ShortcutScope::Launcher,
            action: "Toggle Launcher",
            binding: launcher_binding,
            default_binding: "Alt+Space",
            source: launcher_source,
            resettable: launcher_source == ShortcutSource::User,
        },
        shortcut(
            "launcher.actions",
            ShortcutScope::Launcher,
            "Open Actions",
            "Mod+K",
        ),
        shortcut(
            "launcher.enter",
            ShortcutScope::Launcher,
            "Run Selected",
            "Enter",
        ),
        shortcut(
            "studio.settings",
            ShortcutScope::Studio,
            "Open Settings",
            "Mod+,",
        ),
        shortcut(
            "studio.palette",
            ShortcutScope::Studio,
            "Open Command Palette",
            "Mod+Shift+P",
        ),
        shortcut(
            "workflow.move_step_up",
            ShortcutScope::WorkflowBuilder,
            "Move Step Up",
            "Alt+Up",
        ),
        shortcut(
            "workflow.move_step_down",
            ShortcutScope::WorkflowBuilder,
            "Move Step Down",
            "Alt+Down",
        ),
        shortcut(
            "analysis.qa_focus",
            ShortcutScope::AnalysisWorkbench,
            "Focus Q&A",
            "?",
        ),
    ]
}

fn shortcut(
    id: &'static str,
    scope: ShortcutScope,
    action: &'static str,
    default_binding: &'static str,
) -> ShortcutSpec {
    ShortcutSpec {
        id,
        scope,
        action,
        binding: default_binding.to_string(),
        default_binding,
        source: ShortcutSource::Default,
        resettable: false,
    }
}

impl ShortcutScope {
    pub fn label(self) -> &'static str {
        match self {
            Self::Launcher => "Launcher",
            Self::Studio => "Studio",
            Self::WorkflowBuilder => "Workflow Builder",
            Self::AnalysisWorkbench => "Analysis Workbench",
        }
    }
}

impl ShortcutSource {
    pub fn label(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::User => "user",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_marks_user_launcher_hotkey_override() {
        let config = StdConfig {
            launcher_hotkey: "Cmd+Space".to_string(),
            ..StdConfig::default()
        };
        let registry = shortcut_registry(&config);
        let launcher = registry
            .iter()
            .find(|spec| spec.id == "launcher.global.toggle")
            .unwrap();

        assert_eq!(launcher.binding, "Cmd+Space");
        assert_eq!(launcher.default_binding, "Alt+Space");
        assert_eq!(launcher.source, ShortcutSource::User);
        assert!(launcher.resettable);
    }

    #[test]
    fn registry_includes_studio_and_workflow_shortcuts() {
        let ids = shortcut_registry(&StdConfig::default())
            .into_iter()
            .map(|spec| spec.id)
            .collect::<Vec<_>>();

        assert!(ids.contains(&"studio.palette"));
        assert!(ids.contains(&"workflow.move_step_up"));
        assert!(ids.contains(&"analysis.qa_focus"));
    }
}
