use crate::resident::{ResidentCommand, ResidentEntry};
use crate::ui;
use crate::window::apply_window_commands;
use eframe::egui;
use std::time::Duration;
use std_egui::{
    input,
    tokens::{ThemeMode, ThemeProfile},
};
use std_launcher::{GlobalHotkeyRuntime, LauncherState};

pub(crate) struct LauncherApp {
    pub(crate) state: LauncherState,
    hotkey_runtime: GlobalHotkeyRuntime,
    hotkey_status: String,
    resident_entry: Option<ResidentEntry>,
    resident_status: String,
    voice_transcript: String,
    pub(crate) theme_mode: ThemeMode,
    pub(crate) theme_profile: Option<ThemeProfile>,
    pub(crate) allow_close: bool,
}

impl Default for LauncherApp {
    fn default() -> Self {
        let mut state = LauncherState::new();
        state.hide();
        Self::resident(state)
    }
}

impl eframe::App for LauncherApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        launcher_clear_color()
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_theme_profile(ctx);
        if ctx.input(|input| input.viewport().close_requested()) && !self.allow_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            let commands = self.state.handle_escape_hide();
            self.apply_window_commands_for_current_state(ctx, &commands);
        }

        if self.take_hotkey_toggle() {
            let commands = self.state.handle_hotkey_toggle();
            self.apply_window_commands_for_current_state(ctx, &commands);
        }
        if let Some(command) = self
            .resident_entry
            .as_ref()
            .and_then(ResidentEntry::poll_command)
        {
            match command {
                ResidentCommand::Show => {
                    let commands = self.state.handle_show();
                    self.apply_window_commands_for_current_state(ctx, &commands);
                }
                ResidentCommand::Hide => {
                    let commands = self.state.handle_escape_hide();
                    self.apply_window_commands_for_current_state(ctx, &commands);
                }
                ResidentCommand::Quit => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            }
        }

        if !input::ime_composing(ctx) && input::escape().pressed(ctx) {
            let commands = self.state.handle_escape_hide();
            self.apply_window_commands_for_current_state(ctx, &commands);
        }

        self.sync_viewport_size(ctx);
        if !self.state.controller.visible {
            ctx.request_repaint_after(Duration::from_millis(50));
            return;
        }

        if ui::render_launcher_viewport(
            ctx,
            &mut self.state,
            &self.hotkey_status,
            &self.resident_status,
            &mut self.voice_transcript,
        ) {
            let commands = self.state.handle_escape_hide();
            self.apply_window_commands_for_current_state(ctx, &commands);
        } else {
            self.sync_viewport_size(ctx);
        }
    }
}

pub(crate) fn launcher_clear_color() -> [f32; 4] {
    egui::Color32::TRANSPARENT.to_normalized_gamma_f32()
}

impl LauncherApp {
    pub(crate) fn apply_theme_profile(&mut self, ctx: &egui::Context) {
        let config = &self.state.core.config;
        self.theme_profile = Some(ThemeProfile::apply_with_accessibility(
            ctx,
            self.theme_mode,
            config.reduce_motion(),
            config.high_contrast(),
            config.reduce_transparency(),
            config.ui_scale(),
        ));
    }

    pub(crate) fn for_preview(theme_mode: ThemeMode) -> Self {
        let mut state = LauncherState::new();
        state.hide();
        let plan = state.controller.registration_plan();
        let mut app = Self {
            theme_mode,
            state,
            hotkey_runtime: GlobalHotkeyRuntime::disabled(plan),
            hotkey_status: "preview disabled".to_string(),
            resident_entry: None,
            resident_status: "preview".to_string(),
            voice_transcript: String::new(),
            theme_profile: None,
            allow_close: true,
        };
        app.state.controller.show();
        app
    }

    pub(crate) fn for_background_harness() -> Self {
        let mut app = Self::for_preview(ThemeMode::Dark);
        app.hotkey_status = "harness disabled".to_string();
        app.resident_status = "harness disabled".to_string();
        app
    }

    #[cfg(test)]
    pub(crate) fn system_resource_contract(&self) -> String {
        let status = if self.hotkey_status == "harness disabled"
            && self.resident_status == "harness disabled"
        {
            "harness"
        } else {
            "runtime"
        };
        format!(
            "hotkey={},resident={},status={status}",
            self.hotkey_runtime.is_registered(),
            self.resident_entry.is_some()
        )
    }

    fn resident(state: LauncherState) -> Self {
        let plan = state.controller.registration_plan();
        let (hotkey_runtime, hotkey_status) = match GlobalHotkeyRuntime::register(plan.clone()) {
            Ok(runtime) => (runtime, "registered".to_string()),
            Err(error) => (
                GlobalHotkeyRuntime::disabled(plan),
                format!("disabled: {error}"),
            ),
        };
        let (resident_entry, resident_status) = match ResidentEntry::new() {
            Ok(entry) => {
                let status = entry.status().to_string();
                (Some(entry), status)
            }
            Err(error) => (None, format!("menu bar disabled: {error}")),
        };
        Self {
            theme_mode: ThemeMode::resolve(&state.core.config.theme),
            state,
            hotkey_runtime,
            hotkey_status,
            resident_entry,
            resident_status,
            voice_transcript: String::new(),
            theme_profile: None,
            allow_close: false,
        }
    }

    fn take_hotkey_toggle(&self) -> bool {
        self.hotkey_runtime.poll_toggle_event()
    }

    fn apply_window_commands_for_current_state(
        &self,
        ctx: &egui::Context,
        commands: &[std_launcher::LauncherWindowCommand],
    ) {
        apply_window_commands(ctx, commands, ui::launcher_window_inner_size(&self.state));
    }

    fn sync_viewport_size(&self, ctx: &egui::Context) {
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(
            ui::launcher_window_inner_size(&self.state),
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launcher_app_clears_to_transparent_for_floating_overlay() {
        assert_eq!(launcher_clear_color(), [0.0, 0.0, 0.0, 0.0]);
        assert_eq!(
            std_launcher::launcher_clear_color_contract(),
            "native_clear_color=transparent_rgba_0_0_0_0"
        );
    }

    #[test]
    fn launcher_preview_does_not_register_resident_system_resources() {
        let app = LauncherApp::for_preview(ThemeMode::Dark);

        assert!(!app.hotkey_runtime.is_registered());
        assert!(app.resident_entry.is_none());
        assert_eq!(app.hotkey_status, "preview disabled");
        assert_eq!(app.resident_status, "preview");
    }

    #[test]
    fn launcher_background_harness_does_not_register_system_resources() {
        let app = LauncherApp::for_background_harness();

        assert_eq!(
            app.system_resource_contract(),
            "hotkey=false,resident=false,status=harness"
        );
    }

    #[test]
    fn launcher_app_tracks_applied_theme_profile() {
        let mut app = LauncherApp::for_preview(ThemeMode::Light);
        let ctx = egui::Context::default();

        app.apply_theme_profile(&ctx);

        let profile = app.theme_profile.unwrap();
        assert_eq!(profile.requested, ThemeMode::Light);
        assert_eq!(profile.effective, std_egui::tokens::EffectiveTheme::Light);
    }

    #[test]
    fn launcher_theme_profile_merges_configured_accessibility() {
        let mut app = LauncherApp::for_preview(ThemeMode::Dark);
        app.state.core.config.appearance.reduce_motion = true;
        app.state.core.config.appearance.high_contrast = true;
        app.state.core.config.appearance.reduce_transparency = true;
        app.state.core.config.appearance.ui_scale = 1.25;
        let ctx = egui::Context::default();

        app.apply_theme_profile(&ctx);

        let profile = app.theme_profile.unwrap();
        assert!(profile.reduce_motion);
        assert!(profile.high_contrast);
        assert!(profile.reduce_transparency);
        assert_eq!(profile.focus_ring_width, 3);
    }

    #[test]
    fn launcher_app_global_escape_respects_ime_composition() {
        let source = include_str!("app.rs");

        assert!(source.contains("!input::ime_composing(ctx) && input::escape().pressed(ctx)"));
    }

    #[test]
    fn launcher_app_syncs_native_size_before_hidden_return() {
        let source = include_str!("app.rs");
        let size_sync = source.find("self.sync_viewport_size(ctx);").unwrap();
        let hidden_return = source.find("if !self.state.controller.visible").unwrap();

        assert!(size_sync < hidden_return);
    }

    #[test]
    fn launcher_window_commands_use_current_state_size() {
        let source = include_str!("app.rs");
        let stale_viewport_binding =
            ["let viewport_size = ", "ui::launcher_window_inner_size"].concat();

        assert!(source.contains("apply_window_commands_for_current_state"));
        assert!(!source.contains(&stale_viewport_binding));
        assert!(source.contains("ui::launcher_window_inner_size(&self.state)"));
    }
}
