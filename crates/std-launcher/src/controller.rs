use crate::{HotkeyRegistrationPlan, LauncherHotkey};
use std_core::StdConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherController {
    pub hotkey: LauncherHotkey,
    pub visible: bool,
    pub focused: bool,
    pub voice_active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LauncherWindowCommand {
    PositionForPanel,
    ResizeToPanel,
    ResizeToHiddenHost,
    SetVisible(bool),
    Focus,
}

impl LauncherController {
    pub fn new(config: &StdConfig) -> Self {
        Self {
            hotkey: LauncherHotkey::parse(&config.launcher_hotkey).unwrap_or_else(|| {
                LauncherHotkey {
                    modifiers: vec!["Alt".to_string()],
                    key: "Space".to_string(),
                }
            }),
            visible: false,
            focused: false,
            voice_active: false,
        }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
        self.focused = self.visible;
    }

    pub fn show(&mut self) {
        self.visible = true;
        self.focused = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
        self.focused = false;
        self.voice_active = false;
    }

    pub fn start_voice_input(&mut self) {
        self.visible = true;
        self.focused = true;
        self.voice_active = true;
    }

    pub fn finish_voice_input(&mut self) {
        self.voice_active = false;
    }

    pub fn registration_plan(&self) -> HotkeyRegistrationPlan {
        HotkeyRegistrationPlan {
            accelerator: self.hotkey.accelerator(),
            enabled: true,
        }
    }

    pub fn window_commands(
        previous_visible: bool,
        current_visible: bool,
    ) -> Vec<LauncherWindowCommand> {
        if previous_visible == current_visible {
            return Vec::new();
        }
        if current_visible {
            vec![
                LauncherWindowCommand::ResizeToPanel,
                LauncherWindowCommand::PositionForPanel,
                LauncherWindowCommand::SetVisible(true),
                LauncherWindowCommand::Focus,
            ]
        } else {
            vec![
                LauncherWindowCommand::ResizeToHiddenHost,
                LauncherWindowCommand::SetVisible(false),
            ]
        }
    }
}
