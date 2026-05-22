use crate::{ui_metrics, ui_parts::keycap};
use eframe::egui;
use std_egui::{
    i18n, input,
    tokens::{Color, Radius, Space, Text},
    LauncherFeedback, LauncherFeedbackAction,
};
use std_launcher::LauncherState;
use std_types::ActionExecutionStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FeedbackKind {
    Completed,
    Failed,
    Deferred,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FeedbackDetailSummary {
    text: String,
    truncated: bool,
}

const FEEDBACK_DETAIL_MAX_CHARS: usize = 96;

pub(crate) fn render(ui: &mut egui::Ui, state: &mut LauncherState) -> FeedbackRenderResult {
    let Some(feedback) = state.view.feedback.clone() else {
        return FeedbackRenderResult::default();
    };
    let ctx = ui.ctx().clone();
    let mut command = FeedbackCommand::None;
    let response = egui::Frame::new()
        .fill(feedback_fill(&ctx, &feedback))
        .stroke(egui::Stroke::new(1.0, feedback_stroke(&ctx, &feedback)))
        .corner_radius(egui::CornerRadius::same(Radius::md()))
        .inner_margin(egui::Margin::symmetric(Space::sm(), Space::xs()))
        .show(ui, |ui| command = render_contents(ui, state, &feedback))
        .response;
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            feedback_panel_a11y_label(&feedback),
        )
    });
    FeedbackRenderResult { command }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct FeedbackRenderResult {
    pub(crate) command: FeedbackCommand,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum FeedbackCommand {
    #[default]
    None,
    Trigger(usize),
}

fn render_contents(
    ui: &mut egui::Ui,
    state: &LauncherState,
    feedback: &LauncherFeedback,
) -> FeedbackCommand {
    let ctx = ui.ctx().clone();
    let actions_width = feedback_actions_width(feedback);
    let text_width = (ui.available_width() - actions_width - Space::sm() as f32)
        .max(ui_metrics::scale().f32(240.0));
    let mut command = FeedbackCommand::None;
    ui.horizontal(|ui| {
        render_text(ui, &ctx, feedback, text_width);
        ui.add_space(Space::xs() as f32);
        command = render_actions(ui, state, feedback, actions_width);
    });
    command
}

fn render_text(ui: &mut egui::Ui, ctx: &egui::Context, feedback: &LauncherFeedback, width: f32) {
    ui.allocate_ui_with_layout(
        egui::vec2(width, ui_metrics::feedback_text_height()),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            ui.horizontal(|ui| {
                render_status_icon(ui, ctx, feedback);
                ui.label(
                    egui::RichText::new(feedback_title(feedback))
                        .font(Text::body())
                        .color(Color::fg_primary(ctx))
                        .strong(),
                );
            });
            ui.add_sized(
                [width, ui_metrics::feedback_detail_height()],
                egui::Label::new(
                    egui::RichText::new(feedback_detail_summary(feedback).visible_text())
                        .font(Text::footnote())
                        .color(Color::fg_secondary(ctx)),
                )
                .wrap(),
            );
        },
    );
}

fn render_status_icon(ui: &mut egui::Ui, ctx: &egui::Context, feedback: &LauncherFeedback) {
    let (rect, response) =
        ui.allocate_exact_size(ui_metrics::feedback_icon_size(), egui::Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Other,
            ui.is_enabled(),
            feedback_icon_label(feedback),
        )
    });
    let geometry = ui_metrics::feedback_icon_geometry(rect);
    let stroke = egui::Stroke::new(geometry.stroke_width, feedback_stroke(ctx, feedback));
    match feedback_kind(feedback) {
        FeedbackKind::Completed => {
            ui.painter()
                .line_segment([geometry.check_start, geometry.check_mid], stroke);
            ui.painter()
                .line_segment([geometry.check_mid, geometry.check_end], stroke);
        }
        FeedbackKind::Failed => {
            ui.painter()
                .circle_stroke(geometry.center, geometry.radius, stroke);
            ui.painter()
                .line_segment([geometry.cross_a_start, geometry.cross_a_end], stroke);
            ui.painter()
                .line_segment([geometry.cross_b_start, geometry.cross_b_end], stroke);
        }
        FeedbackKind::Deferred => {
            ui.painter()
                .circle_stroke(geometry.center, geometry.radius, stroke);
            ui.painter()
                .line_segment([geometry.alert_top, geometry.alert_mid], stroke);
            ui.painter()
                .circle_filled(geometry.alert_dot, geometry.dot_radius, stroke.color);
        }
    }
}

fn render_actions(
    ui: &mut egui::Ui,
    state: &LauncherState,
    feedback: &LauncherFeedback,
    width: f32,
) -> FeedbackCommand {
    let actions = feedback.actions();
    let mut command = FeedbackCommand::None;
    ui.allocate_ui_with_layout(
        egui::vec2(width, ui_metrics::feedback_text_height()),
        egui::Layout::right_to_left(egui::Align::Center),
        |ui| {
            for (index, action) in actions.into_iter().enumerate() {
                let selected = state.focus_section == std_launcher::LauncherFocusSection::Feedback
                    && state.view.selected_feedback_action == index;
                match action {
                    LauncherFeedbackAction::Copy => {
                        if feedback_button(
                            ui,
                            i18n::t("launcher.feedback.copy"),
                            feedback_action_a11y_label(feedback, LauncherFeedbackAction::Copy),
                            selected,
                        )
                        .clicked()
                        {
                            command = FeedbackCommand::Trigger(index);
                        }
                    }
                    LauncherFeedbackAction::Retry => {
                        if feedback_button(
                            ui,
                            i18n::t("launcher.feedback.retry"),
                            feedback_action_a11y_label(feedback, LauncherFeedbackAction::Retry),
                            selected,
                        )
                        .clicked()
                        {
                            command = FeedbackCommand::Trigger(index);
                        }
                    }
                    LauncherFeedbackAction::OpenStudio => {
                        if feedback_button(
                            ui,
                            i18n::t("launcher.feedback.open_studio"),
                            feedback_action_a11y_label(
                                feedback,
                                LauncherFeedbackAction::OpenStudio,
                            ),
                            selected,
                        )
                        .clicked()
                        {
                            command = FeedbackCommand::Trigger(index);
                        }
                    }
                }
            }
        },
    );
    command
}

fn feedback_actions_width(feedback: &LauncherFeedback) -> f32 {
    let count = feedback.actions().len() as f32;
    ui_metrics::scale().f32(count * 76.0)
}

fn feedback_button(
    ui: &mut egui::Ui,
    label: &str,
    a11y_label: String,
    selected: bool,
) -> egui::Response {
    let ctx = ui.ctx().clone();
    let fill = if selected {
        Color::accent_weak(&ctx)
    } else {
        Color::bg_surface_0(&ctx)
    };
    let stroke = if selected {
        Color::accent_base(&ctx)
    } else {
        Color::stroke_divider(&ctx)
    };
    let frame = egui::Frame::new()
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, stroke))
        .corner_radius(egui::CornerRadius::same(Radius::sm()))
        .inner_margin(egui::Margin::symmetric(Space::xs(), Space::two_xs()));

    let response = frame
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(label)
                        .font(Text::caption())
                        .color(Color::fg_primary(&ctx)),
                );
                if selected {
                    keycap(ui, &input::enter().label());
                }
            });
        })
        .response
        .interact(egui::Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            a11y_label.clone(),
        )
    });
    if selected {
        return response.on_hover_text(input::enter().label());
    }
    response
}

fn feedback_kind(feedback: &LauncherFeedback) -> FeedbackKind {
    match feedback.status {
        ActionExecutionStatus::Completed => FeedbackKind::Completed,
        ActionExecutionStatus::Failed => FeedbackKind::Failed,
        ActionExecutionStatus::NeedsExternalRunner => FeedbackKind::Deferred,
    }
}

fn feedback_title(feedback: &LauncherFeedback) -> String {
    match feedback_kind(feedback) {
        FeedbackKind::Completed => feedback.title.clone(),
        FeedbackKind::Failed => format!("{}: {}", feedback.title, feedback.action_name),
        FeedbackKind::Deferred => format!("{}: {}", feedback.title, feedback.action_name),
    }
}

fn feedback_icon_label(feedback: &LauncherFeedback) -> &'static str {
    match feedback_kind(feedback) {
        FeedbackKind::Completed => i18n::t("launcher.feedback.icon.completed"),
        FeedbackKind::Failed => i18n::t("launcher.feedback.icon.failed"),
        FeedbackKind::Deferred => i18n::t("launcher.feedback.icon.deferred"),
    }
}

fn feedback_action_a11y_label(
    feedback: &LauncherFeedback,
    action: LauncherFeedbackAction,
) -> String {
    i18n::t("launcher.feedback.action.a11y")
        .replace("{action}", feedback_action_label(action))
        .replace("{target}", &feedback.action_name)
        .replace("{status}", feedback.status_label())
}

fn feedback_panel_a11y_label(feedback: &LauncherFeedback) -> String {
    let actions = feedback
        .actions()
        .into_iter()
        .map(feedback_action_label)
        .collect::<Vec<_>>()
        .join(", ");
    i18n::t("launcher.feedback.panel.a11y")
        .replace("{status}", feedback.status_label())
        .replace("{target}", &feedback.action_name)
        .replace("{actions}", &actions)
}

fn feedback_action_label(action: LauncherFeedbackAction) -> &'static str {
    match action {
        LauncherFeedbackAction::Copy => i18n::t("launcher.feedback.copy"),
        LauncherFeedbackAction::Retry => i18n::t("launcher.feedback.retry"),
        LauncherFeedbackAction::OpenStudio => i18n::t("launcher.feedback.open_studio"),
    }
}

fn feedback_detail_summary(feedback: &LauncherFeedback) -> FeedbackDetailSummary {
    let mut truncated = false;
    let mut text = feedback
        .detail
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            if index >= 2 {
                truncated = true;
                return None;
            }
            Some(line.trim())
        })
        .collect::<Vec<_>>()
        .join(" ");
    if text.chars().count() > FEEDBACK_DETAIL_MAX_CHARS {
        text = text
            .chars()
            .take(FEEDBACK_DETAIL_MAX_CHARS)
            .collect::<String>();
        truncated = true;
    }
    FeedbackDetailSummary { text, truncated }
}

impl FeedbackDetailSummary {
    fn visible_text(&self) -> String {
        if self.truncated {
            format!("{} {}", self.text, i18n::t("launcher.feedback.truncated"))
        } else {
            self.text.clone()
        }
    }
}

fn feedback_fill(ctx: &egui::Context, feedback: &LauncherFeedback) -> egui::Color32 {
    match feedback_kind(feedback) {
        FeedbackKind::Completed => Color::status_weak(ctx, Color::status_success(ctx)),
        FeedbackKind::Failed => Color::status_weak(ctx, Color::status_danger(ctx)),
        FeedbackKind::Deferred => Color::status_weak(ctx, Color::status_warning(ctx)),
    }
}

fn feedback_stroke(ctx: &egui::Context, feedback: &LauncherFeedback) -> egui::Color32 {
    match feedback_kind(feedback) {
        FeedbackKind::Completed => Color::status_success(ctx),
        FeedbackKind::Failed => Color::status_danger(ctx),
        FeedbackKind::Deferred => Color::status_warning(ctx),
    }
}

#[cfg(test)]
#[path = "ui_feedback_tests.rs"]
mod ui_feedback_tests;
