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
        let viewport_size = ui::launcher_window_inner_size(&self.state);
        if ctx.input(|input| input.viewport().close_requested()) && !self.allow_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            apply_window_commands(ctx, &self.state.handle_escape_hide(), viewport_size);
        }

        if self.take_hotkey_toggle() {
            apply_window_commands(ctx, &self.state.handle_hotkey_toggle(), viewport_size);
        }
        if let Some(command) = self
            .resident_entry
            .as_ref()
            .and_then(ResidentEntry::poll_command)
        {
            match command {
                ResidentCommand::Show => {
                    apply_window_commands(ctx, &self.state.handle_show(), viewport_size);
                }
                ResidentCommand::Hide => {
                    apply_window_commands(ctx, &self.state.handle_escape_hide(), viewport_size);
                }
                ResidentCommand::Quit => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            }
        }

        if input::escape().pressed(ctx) {
            apply_window_commands(ctx, &self.state.handle_escape_hide(), viewport_size);
        }

        if !self.state.controller.visible {
            ctx.request_repaint_after(Duration::from_millis(50));
            return;
        }

        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(viewport_size));
        if ui::render_launcher_viewport(
            ctx,
            &mut self.state,
            &self.hotkey_status,
            &self.resident_status,
            &mut self.voice_transcript,
        ) {
            apply_window_commands(ctx, &self.state.handle_escape_hide(), viewport_size);
        }
    }
}

fn launcher_clear_color() -> [f32; 4] {
    egui::Color32::TRANSPARENT.to_normalized_gamma_f32()
}

impl LauncherApp {
    pub(crate) fn apply_theme_profile(&mut self, ctx: &egui::Context) {
        self.theme_profile = Some(ThemeProfile::apply(ctx, self.theme_mode));
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launcher_app_clears_to_transparent_for_floating_overlay() {
        assert_eq!(launcher_clear_color(), [0.0, 0.0, 0.0, 0.0]);
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
    fn launcher_app_tracks_applied_theme_profile() {
        let mut app = LauncherApp::for_preview(ThemeMode::Light);
        let ctx = egui::Context::default();

        app.apply_theme_profile(&ctx);

        let profile = app.theme_profile.unwrap();
        assert_eq!(profile.requested, ThemeMode::Light);
        assert_eq!(profile.effective, std_egui::tokens::EffectiveTheme::Light);
    }
}
