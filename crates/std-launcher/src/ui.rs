use crate::{
    ui_action_bar, ui_action_panel, ui_feedback, ui_keyboard, ui_metrics, ui_parts::quiet_button,
    ui_results, ui_search,
};
use eframe::egui;
use std_egui::{
    i18n, input,
    tokens::{Color, Elevation, Radius, Space},
};
use std_launcher::LauncherState;

pub(crate) fn render_launcher_viewport(
    ctx: &egui::Context,
    state: &mut LauncherState,
    hotkey_status: &str,
    resident_status: &str,
    voice_transcript: &mut String,
) -> bool {
    let mut hide_requested = false;
    egui::CentralPanel::default()
        .frame(launcher_viewport_frame())
        .show(ctx, |ui| {
            hide_requested = render_launcher_overlay(
                ui,
                state,
                hotkey_status,
                resident_status,
                voice_transcript,
            );
        });
    hide_requested
}

pub(crate) fn launcher_viewport_frame() -> egui::Frame {
    egui::Frame::NONE.fill(egui::Color32::TRANSPARENT)
}

pub(crate) fn launcher_initial_window_inner_size() -> egui::Vec2 {
    ui_metrics::initial_window_inner_size()
}

pub(crate) fn launcher_window_inner_size(state: &LauncherState) -> egui::Vec2 {
    ui_metrics::window_inner_size(state)
}

pub(crate) fn render_launcher_overlay(
    ui: &mut egui::Ui,
    state: &mut LauncherState,
    hotkey_status: &str,
    resident_status: &str,
    voice_transcript: &mut String,
) -> bool {
    let available = ui.max_rect();
    let body_height = ui_metrics::body_height(state, available.height());
    let rect = ui_metrics::panel_rect(available, state);

    let mut hide_requested = false;
    ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
        hide_requested = render_launcher_panel(
            ui,
            state,
            hotkey_status,
            resident_status,
            voice_transcript,
            body_height,
        );
    });
    hide_requested
}

pub(crate) fn render_launcher_panel(
    ui: &mut egui::Ui,
    state: &mut LauncherState,
    hotkey_status: &str,
    resident_status: &str,
    voice_transcript: &mut String,
    body_height: f32,
) -> bool {
    let mut hide_requested = false;
    let ctx = ui.ctx().clone();
    let panel_rect = ui.max_rect();
    ui.set_min_width(panel_rect.width());
    ui.set_min_height(panel_rect.height());
    egui::Frame::new()
        .fill(Color::bg_surface_0(&ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_border(&ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::xl()))
        .shadow(Elevation::level_3(&ctx))
        .inner_margin(egui::Margin::same(
            ui_metrics::panel_inner_padding_for_state(state) as i8,
        ))
        .show(ui, |ui| {
            let padding = ui_metrics::panel_inner_padding_for_state(state);
            ui.set_min_height(panel_rect.height() - padding * 2.0);
            ui.set_width(panel_rect.width() - padding * 2.0);
            let collapsed = !ui_metrics::panel_is_expanded(state);
            ui_search::render_search_bar(ui, state, collapsed, &mut hide_requested);
            if !ui_metrics::panel_is_expanded(state) {
                return;
            }
            ui.add_space(Space::xs() as f32);
            hide_requested |= render_body(ui, state, body_height);
            ui.add_space(Space::xs() as f32);
            let action_bar = ui_action_bar::render(ui, state, hotkey_status, resident_status);
            match action_bar.command {
                ui_action_bar::ActionBarCommand::CancelExecuting => state.cancel_executing(),
                ui_action_bar::ActionBarCommand::MoveExecutingToBackground => {
                    state.move_executing_to_background();
                    hide_requested = true;
                }
                ui_action_bar::ActionBarCommand::None => {}
            }
            render_voice(ui, state, voice_transcript);
            let feedback = ui_feedback::render(ui, state);
            match feedback.command {
                ui_feedback::FeedbackCommand::Trigger(trigger) => {
                    state.focus_section = std_launcher::LauncherFocusSection::Feedback;
                    state.view.selected_feedback_action = trigger.index;
                    if let Some(execution) = state.trigger_feedback_action() {
                        if trigger.action == std_egui::LauncherFeedbackAction::Copy {
                            ui.ctx().copy_text(execution.message);
                        }
                    }
                }
                ui_feedback::FeedbackCommand::None => {}
            }
            let action_panel = ui_action_panel::render(ui.ctx(), action_bar.rect, state);
            match action_panel.command {
                ui_action_panel::ActionPanelCommand::Triggered(trigger) => {
                    let execution = trigger.execution;
                    hide_requested |= ui_keyboard::execution_hides_launcher(&execution);
                    if trigger.copy_to_clipboard {
                        ui.ctx().copy_text(execution.message);
                    }
                }
                ui_action_panel::ActionPanelCommand::None => {}
            }
        });
    hide_requested
}

fn render_body(ui: &mut egui::Ui, state: &mut LauncherState, max_height: f32) -> bool {
    ui_results::render(ui, state, max_height)
}

fn render_voice(ui: &mut egui::Ui, state: &mut LauncherState, voice_transcript: &mut String) {
    if !state.controller.voice_active {
        return;
    }
    let ctx = ui.ctx().clone();
    ui.add_space(Space::xs() as f32);
    egui::Frame::new()
        .fill(Color::bg_surface_2(&ctx))
        .corner_radius(egui::CornerRadius::same(Radius::md()))
        .inner_margin(egui::Margin::same(Space::xs()))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(i18n::t("launcher.voice.label"))
                        .color(Color::fg_secondary(&ctx)),
                );
                let voice_response = ui.add_sized(
                    [
                        ui_metrics::voice_input_width(ui.available_width()),
                        ui_metrics::voice_input_height(),
                    ],
                    egui::TextEdit::singleline(voice_transcript)
                        .hint_text(i18n::t("launcher.voice.placeholder")),
                );
                voice_response.request_focus();
                voice_response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::TextEdit,
                        ui.is_enabled(),
                        voice_input_a11y_label(voice_transcript),
                    )
                });
                if voice_apply_pressed(&ctx) {
                    state.apply_voice_transcript(voice_transcript.as_str());
                }
                if quiet_button(ui, i18n::t("launcher.voice.apply")).clicked() {
                    state.apply_voice_transcript(voice_transcript.as_str());
                }
            });
        });
}

fn voice_apply_pressed(ctx: &egui::Context) -> bool {
    !input::ime_composing(ctx) && input::enter().pressed(ctx)
}

fn voice_input_a11y_label(transcript: &str) -> String {
    let value = if transcript.trim().is_empty() {
        i18n::t("launcher.voice.empty_value")
    } else {
        transcript.trim()
    };
    i18n::t("launcher.voice.input.a11y")
        .replace("{label}", i18n::t("launcher.voice.label"))
        .replace("{value}", value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launcher_viewport_frame_is_transparent_and_unstyled() {
        let frame = launcher_viewport_frame();

        assert_eq!(frame.fill, egui::Color32::TRANSPARENT);
        assert_eq!(frame.stroke, egui::Stroke::NONE);
        assert_eq!(
            std_launcher::launcher_viewport_frame_contract(),
            "viewport_frame=transparent_fill,no_stroke"
        );
    }

    #[test]
    fn launcher_panel_forces_frame_to_computed_viewport_size() {
        let source = include_str!("ui.rs");

        assert!(source.contains("ui.set_min_width(panel_rect.width())"));
        assert!(source.contains("ui.set_min_height(panel_rect.height())"));
        assert!(source.contains("ui.set_min_height(panel_rect.height() - padding * 2.0)"));
    }

    #[test]
    fn launcher_panel_handles_action_panel_trigger_results() {
        let source = include_str!("ui.rs");
        let production_source = source.split("#[cfg(test)]").next().unwrap();

        assert!(
            production_source.contains("ui_action_panel::render(ui.ctx(), action_bar.rect, state)")
        );
        assert!(production_source.contains("ActionPanelCommand::Triggered(trigger)"));
        assert!(production_source.contains("ui_keyboard::execution_hides_launcher(&execution)"));
        assert!(production_source.contains("trigger.copy_to_clipboard"));
        assert!(production_source.contains("ui.ctx().copy_text(execution.message)"));
    }

    #[test]
    fn launcher_primary_structure_has_no_separate_preview_panel() {
        let source = include_str!("ui.rs");
        let production_source = source.split("#[cfg(test)]").next().unwrap();

        assert!(!production_source.contains("ui_preview_panel"));
    }

    #[test]
    fn launcher_feedback_mouse_copy_is_limited_to_copy_action() {
        let source = include_str!("ui.rs");
        let production_source = source.split("#[cfg(test)]").next().unwrap();
        let feedback_branch = production_source
            .split("FeedbackCommand::Trigger(trigger)")
            .nth(1)
            .and_then(|body| body.split("FeedbackCommand::None").next())
            .unwrap();

        assert!(feedback_branch.contains("trigger.index"));
        assert!(feedback_branch.contains("trigger.action"));
        assert!(feedback_branch.contains("LauncherFeedbackAction::Copy"));
        assert!(feedback_branch.contains("ui.ctx().copy_text(execution.message)"));
    }

    #[test]
    fn voice_input_a11y_label_exposes_value() {
        assert_eq!(
            voice_input_a11y_label("open terminal"),
            i18n::t("launcher.voice.input.a11y")
                .replace("{label}", i18n::t("launcher.voice.label"))
                .replace("{value}", "open terminal")
        );
        assert_eq!(
            voice_input_a11y_label("  "),
            i18n::t("launcher.voice.input.a11y")
                .replace("{label}", i18n::t("launcher.voice.label"))
                .replace("{value}", i18n::t("launcher.voice.empty_value"))
        );
    }

    #[test]
    fn voice_input_requests_egui_focus_when_visible() {
        let source = include_str!("ui.rs");
        let production_source = source.split("#[cfg(test)]").next().unwrap();
        let voice_branch = production_source
            .split("let voice_response = ui.add_sized")
            .nth(1)
            .and_then(|body| body.split("voice_response.widget_info").next())
            .unwrap();

        assert!(voice_branch.contains("voice_response.request_focus();"));
    }

    #[test]
    fn voice_input_textedit_owns_text_input() {
        let ctx = egui::Context::default();
        let mut state = LauncherState::new();
        state.start_voice_input();
        let mut voice_transcript = String::new();

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_voice(ui, &mut state, &mut voice_transcript);
            });
        });
        let _ = ctx.run(voice_text_input("open notes"), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_voice(ui, &mut state, &mut voice_transcript);
            });
        });

        assert_eq!(voice_transcript, "open notes");
    }

    #[test]
    fn voice_enter_applies_transcript_without_triggering_results() {
        let ctx = egui::Context::default();
        let mut state = LauncherState::new();
        state.start_voice_input();
        let mut voice_transcript = "open notes".to_string();

        let _ = ctx.run(enter_key_input(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_voice(ui, &mut state, &mut voice_transcript);
            });
        });

        assert!(!state.controller.voice_active);
        assert_eq!(state.view.query, "open notes");
        assert!(state.view.last_execution.is_none());
    }

    #[test]
    fn voice_enter_is_owned_by_ime_while_composing() {
        let source = include_str!("ui.rs");
        let production_source = source.split("#[cfg(test)]").next().unwrap();
        let voice_apply_branch = production_source
            .split("fn voice_apply_pressed")
            .nth(1)
            .unwrap();

        assert!(voice_apply_branch.contains("!input::ime_composing(ctx)"));
        assert!(voice_apply_branch.contains("input::enter().pressed(ctx)"));
    }

    fn voice_text_input(text: &str) -> egui::RawInput {
        egui::RawInput {
            events: vec![egui::Event::Text(text.to_string())],
            ..Default::default()
        }
    }

    fn enter_key_input() -> egui::RawInput {
        egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::Enter,
                physical_key: Some(egui::Key::Enter),
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..Default::default()
        }
    }
}
