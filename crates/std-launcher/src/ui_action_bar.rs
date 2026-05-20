use crate::{
    ui_metrics,
    ui_parts::{keycap, quiet_label, weak_status_fill},
};
use eframe::egui;
use std_egui::{
    i18n, input,
    tokens::{Color, Radius, Space, Text},
    LauncherFeedback, LauncherPhase,
};
use std_launcher::{LauncherPerformanceReport, LauncherState};
use std_types::ActionExecutionStatus;

pub(crate) fn render(
    ui: &mut egui::Ui,
    state: &LauncherState,
    hotkey_status: &str,
    resident_status: &str,
) -> egui::Rect {
    let ctx = ui.ctx().clone();
    egui::Frame::new()
        .fill(Color::bg_surface_1(&ctx))
        .corner_radius(egui::CornerRadius::same(Radius::md()))
        .inner_margin(egui::Margin::symmetric(Space::xs(), Space::two_xs()))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let right_width = 272.0_f32.min(ui.available_width() * 0.48);
                let left_width = (ui.available_width() - right_width - Space::xs() as f32)
                    .max(ui_metrics::scale().f32(160.0));
                ui.allocate_ui_with_layout(
                    egui::vec2(left_width, ui_metrics::action_bar_content_height()),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| render_action_summary(ui, state, left_width),
                );
                ui.allocate_ui_with_layout(
                    egui::vec2(right_width, ui_metrics::action_bar_content_height()),
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| render_status_hints(ui, state, &ctx, hotkey_status, resident_status),
                );
            });
        })
        .response
        .rect
}

pub(crate) fn render_feedback(ui: &mut egui::Ui, state: &LauncherState) {
    let ctx = ui.ctx().clone();
    let feedback = state.view.feedback.as_ref();
    if feedback.is_none() {
        return;
    }
    egui::Frame::new()
        .fill(feedback_fill(&ctx, feedback))
        .stroke(egui::Stroke::new(1.0, feedback_stroke(&ctx, feedback)))
        .corner_radius(egui::CornerRadius::same(Radius::md()))
        .inner_margin(egui::Margin::symmetric(Space::sm(), Space::xs()))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                if let Some(feedback) = feedback {
                    render_feedback_text(ui, &ctx, feedback);
                }
                render_performance(ui, &state.performance_report());
            });
        });
}

fn render_status_hints(
    ui: &mut egui::Ui,
    state: &LauncherState,
    ctx: &egui::Context,
    hotkey_status: &str,
    resident_status: &str,
) {
    match action_bar_hint_mode(state) {
        ActionBarHintMode::Cancel => {
            keycap(ui, "Ctrl+C");
            quiet_label(ui, i18n::t("launcher.action.cancel"));
        }
        ActionBarHintMode::RunActions => {
            keycap(ui, &input::launcher_action_panel().label());
            quiet_label(ui, i18n::t("launcher.action.actions"));
            keycap(ui, "Enter");
            quiet_label(ui, i18n::t("launcher.action.run"));
        }
    }
    quiet_label(ui, hotkey_status);
    ui.label(
        egui::RichText::new(resident_status)
            .font(Text::caption())
            .color(Color::fg_secondary(ctx)),
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ActionBarHintMode {
    RunActions,
    Cancel,
}

fn action_bar_hint_mode(state: &LauncherState) -> ActionBarHintMode {
    if state.view.phase == LauncherPhase::Executing {
        ActionBarHintMode::Cancel
    } else {
        ActionBarHintMode::RunActions
    }
}

fn render_action_summary(ui: &mut egui::Ui, state: &LauncherState, max_width: f32) {
    let ctx = ui.ctx().clone();
    if state.view.phase == LauncherPhase::Executing {
        ui.horizontal(|ui| {
            ui.spinner();
            ui.label(
                egui::RichText::new(i18n::t("launcher.action.executing"))
                    .font(Text::footnote())
                    .color(Color::fg_primary(&ctx))
                    .strong(),
            );
        });
        return;
    }
    if let Some(preview) = state.view.preview.as_ref() {
        ui.add_sized(
            [max_width * 0.34, ui_metrics::action_summary_label_height()],
            egui::Label::new(
                egui::RichText::new(&preview.title)
                    .font(Text::footnote())
                    .color(Color::fg_primary(&ctx))
                    .strong(),
            )
            .truncate(),
        );
        ui.add_sized(
            [max_width * 0.62, ui_metrics::action_summary_label_height()],
            egui::Label::new(
                egui::RichText::new(&preview.subtitle)
                    .font(Text::caption())
                    .color(Color::fg_secondary(&ctx)),
            )
            .truncate(),
        );
        return;
    }
    ui.add_sized(
        [max_width, ui_metrics::action_summary_label_height()],
        egui::Label::new(
            egui::RichText::new(i18n::t("launcher.action.command_hint"))
                .font(Text::footnote())
                .color(Color::fg_secondary(&ctx)),
        )
        .truncate(),
    );
}

fn render_feedback_text(ui: &mut egui::Ui, ctx: &egui::Context, feedback: &LauncherFeedback) {
    ui.label(
        egui::RichText::new(feedback_marker(feedback))
            .font(Text::body())
            .color(feedback_stroke(ctx, Some(feedback))),
    );
    ui.label(
        egui::RichText::new(&feedback.title)
            .font(Text::body())
            .color(Color::fg_primary(ctx))
            .strong(),
    );
    ui.label(
        egui::RichText::new(&feedback.action_name)
            .font(Text::footnote())
            .color(Color::fg_primary(ctx)),
    );
    ui.label(
        egui::RichText::new(&feedback.detail)
            .font(Text::footnote())
            .color(Color::fg_secondary(ctx)),
    );
}

fn render_performance(ui: &mut egui::Ui, report: &LauncherPerformanceReport) {
    let ctx = ui.ctx().clone();
    let text = format!(
        "{}ms search  {}ms preview  {}ms action",
        report.last_search_ms, report.last_preview_ms, report.last_trigger_ms
    );
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.label(
            egui::RichText::new(text)
                .font(Text::code())
                .color(Color::fg_secondary(&ctx)),
        );
    });
}

fn feedback_fill(ctx: &egui::Context, feedback: Option<&LauncherFeedback>) -> egui::Color32 {
    match feedback.map(|feedback| &feedback.status) {
        Some(ActionExecutionStatus::Completed) => weak_status_fill(ctx, Color::status_success(ctx)),
        Some(ActionExecutionStatus::Failed) => weak_status_fill(ctx, Color::status_danger(ctx)),
        Some(ActionExecutionStatus::NeedsExternalRunner) => {
            weak_status_fill(ctx, Color::status_warning(ctx))
        }
        None => Color::bg_surface_1(ctx),
    }
}

fn feedback_stroke(ctx: &egui::Context, feedback: Option<&LauncherFeedback>) -> egui::Color32 {
    match feedback.map(|feedback| &feedback.status) {
        Some(ActionExecutionStatus::Completed) => Color::status_success(ctx),
        Some(ActionExecutionStatus::Failed) => Color::status_danger(ctx),
        Some(ActionExecutionStatus::NeedsExternalRunner) => Color::status_warning(ctx),
        None => Color::stroke_divider(ctx),
    }
}

fn feedback_marker(feedback: &LauncherFeedback) -> &'static str {
    match feedback.status {
        ActionExecutionStatus::Completed => "OK",
        ActionExecutionStatus::Failed => "ERR",
        ActionExecutionStatus::NeedsExternalRunner => "WAIT",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_bar_hint_switches_to_cancel_while_executing() {
        let mut state = LauncherState::new();

        assert_eq!(action_bar_hint_mode(&state), ActionBarHintMode::RunActions);

        state.view.preview_executing();

        assert_eq!(action_bar_hint_mode(&state), ActionBarHintMode::Cancel);
    }
}
