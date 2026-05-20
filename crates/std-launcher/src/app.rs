use crate::resident::{ResidentCommand, ResidentEntry};
use crate::ui;
use crate::window::apply_window_commands;
use eframe::egui;
use std::time::Duration;
use std_egui::tokens::{apply_theme, ThemeMode};
use std_launcher::{GlobalHotkeyRuntime, LauncherState};

pub(crate) struct LauncherApp {
    pub(crate) state: LauncherState,
    hotkey_runtime: GlobalHotkeyRuntime,
    hotkey_status: String,
    resident_entry: Option<ResidentEntry>,
    resident_status: String,
    voice_transcript: String,
    pub(crate) theme_mode: ThemeMode,
    pub(crate) allow_close: bool,
}

impl Default for LauncherApp {
    fn default() -> Self {
        let mut state = LauncherState::new();
        state.hide();
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
            allow_close: false,
        }
    }
}

impl eframe::App for LauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        apply_theme(ctx, self.theme_mode);
        if ctx.input(|input| input.viewport().close_requested()) && !self.allow_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            apply_window_commands(ctx, &self.state.handle_escape_hide());
        }

        if self.take_hotkey_toggle() {
            apply_window_commands(ctx, &self.state.handle_hotkey_toggle());
        }
        if let Some(command) = self
            .resident_entry
            .as_ref()
            .and_then(ResidentEntry::poll_command)
        {
            match command {
                ResidentCommand::Show => apply_window_commands(ctx, &self.state.handle_show()),
                ResidentCommand::Hide => {
                    apply_window_commands(ctx, &self.state.handle_escape_hide());
                }
                ResidentCommand::Quit => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            }
        }

        if ctx.input(|input| input.key_pressed(egui::Key::Escape)) {
            apply_window_commands(ctx, &self.state.handle_escape_hide());
        }

        if !self.state.controller.visible {
            ctx.request_repaint_after(Duration::from_millis(50));
            return;
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(egui::Color32::TRANSPARENT))
            .show(ctx, |ui| {
                if ui::render_launcher_overlay(
                    ui,
                    &mut self.state,
                    &self.hotkey_status,
                    &self.resident_status,
                    &mut self.voice_transcript,
                ) {
                    apply_window_commands(ctx, &self.state.handle_escape_hide());
                }
            });
    }
}

impl LauncherApp {
    pub(crate) fn for_preview(theme_mode: ThemeMode) -> Self {
        let mut app = Self {
            theme_mode,
            allow_close: true,
            ..Self::default()
        };
        app.state.controller.show();
        app
    }

    fn take_hotkey_toggle(&self) -> bool {
        self.hotkey_runtime.poll_toggle_event()
    }
}
