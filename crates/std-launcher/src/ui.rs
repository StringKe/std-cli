use crate::{
    ui_action_panel,
    ui_empty::{self, EmptyAction},
    ui_parts::{keycap, quiet_button, quiet_label, surface_frame, weak_status_fill},
};
use eframe::egui;
use std_egui::{
    a11y::AccessibilityContext,
    i18n, input,
    tokens::{self, Color, Radius, Space, Text},
    LauncherFeedback,
};
use std_launcher::{LauncherKey, LauncherPerformanceReport, LauncherState};
use std_types::{ActionExecutionStatus, ActionType, SearchResult};

pub(crate) fn render_launcher_panel(
    ui: &mut egui::Ui,
    state: &mut LauncherState,
    hotkey_status: &str,
    resident_status: &str,
    voice_transcript: &mut String,
) -> bool {
    let mut hide_requested = false;
    let ctx = ui.ctx().clone();
    let panel_rect = ui.max_rect();
    ui.painter().rect_stroke(
        panel_rect,
        egui::CornerRadius::same(Radius::XL),
        egui::Stroke::new(1.0, Color::stroke_border(&ctx)),
        egui::StrokeKind::Inside,
    );
    render_search_bar(ui, state, &mut hide_requested);
    ui.add_space(Space::XS as f32);
    let body_height = (ui.available_height() - 42.0).clamp(128.0, 260.0);
    render_body(ui, state, body_height);
    ui.add_space(Space::XS as f32);
    let action_bar_rect = render_action_bar(ui, state, hotkey_status, resident_status);
    render_voice(ui, state, voice_transcript);
    render_feedback(ui, state);
    ui_action_panel::render(ui.ctx(), action_bar_rect, state);
    hide_requested
}

fn render_search_bar(ui: &mut egui::Ui, state: &mut LauncherState, hide_requested: &mut bool) {
    let ctx = ui.ctx().clone();
    egui::Frame::new()
        .fill(Color::bg_surface_1(&ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_border(&ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::LG))
        .inner_margin(egui::Margin::symmetric(Space::MD, Space::SM))
        .show(ui, |ui| {
            ui.set_min_height(44.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(">")
                        .font(Text::headline())
                        .color(Color::fg_secondary(&ctx)),
                );
                let response = ui.add_sized(
                    [ui.available_width() - 92.0, 36.0],
                    egui::TextEdit::singleline(&mut state.view.query)
                        .hint_text(i18n::t("launcher.search.placeholder"))
                        .font(Text::headline()),
                );
                response.request_focus();
                let a11y = AccessibilityContext::from_env();
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::TextEdit,
                        ui.is_enabled(),
                        a11y.launcher_search_label(&state.view.query),
                    )
                });
                draw_focus_ring(ui, response.rect, Radius::LG, a11y.focus_ring_width());
                if response.changed() {
                    state.update_query(state.view.query.clone());
                }
                if quiet_button(ui, "Esc").clicked() {
                    *hide_requested = true;
                }
            });
        });

    if tokens::ime_composing(&ctx) {
        return;
    }
    if ui.input(|input| input.key_pressed(egui::Key::ArrowDown)) {
        state.handle_keyboard_input(LauncherKey::ArrowDown, false);
    }
    if ui.input(|input| input.key_pressed(egui::Key::ArrowUp)) {
        state.handle_keyboard_input(LauncherKey::ArrowUp, false);
    }
    if ui.input(|input| input.key_pressed(egui::Key::Enter)) {
        state.handle_keyboard_input_by_user(LauncherKey::Enter, false);
    }
    if input::launcher_action_panel().pressed(&ctx) {
        state.handle_keyboard_input_by_user(LauncherKey::ActionPanel, false);
    }
}

fn render_body(ui: &mut egui::Ui, state: &mut LauncherState, max_height: f32) {
    surface_frame(ui.ctx()).show(ui, |ui| {
        render_results(ui, state, max_height);
    });
}

fn render_results(ui: &mut egui::Ui, state: &mut LauncherState, max_height: f32) {
    let ctx = ui.ctx().clone();
    section_header(
        ui,
        "Results",
        &format!("{} matches", state.view.results.len()),
    );
    let mut clicked = None;
    let mut last_group = String::new();
    egui::ScrollArea::vertical()
        .id_salt("launcher_results")
        .max_height(max_height)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if state.view.results.is_empty() {
                if let Some(EmptyAction::AskAi(query)) =
                    ui_empty::render_no_results(ui, &state.view.query)
                {
                    state.update_query(query);
                }
                return;
            }
            for (index, result) in state.view.results.iter().enumerate() {
                let group = action_group(result);
                if group != last_group {
                    group_header(ui, &group);
                    last_group = group;
                }
                if result_row(
                    ui,
                    result,
                    index,
                    state.view.results.len(),
                    index == state.view.selected,
                )
                .clicked()
                {
                    clicked = Some(index);
                }
                ui.add_space(Space::TWO_XS as f32);
            }
        });

    if let Some(index) = clicked {
        state.view.selected = index;
        state.view.refresh_preview(&state.core);
    }
    let _ = ctx;
}

fn result_row(
    ui: &mut egui::Ui,
    result: &SearchResult,
    index: usize,
    total: usize,
    selected: bool,
) -> egui::Response {
    let ctx = ui.ctx().clone();
    let a11y = AccessibilityContext::from_env();
    let fill = if selected {
        Color::accent_weak(&ctx)
    } else {
        egui::Color32::TRANSPARENT
    };
    egui::Frame::new()
        .fill(fill)
        .corner_radius(egui::CornerRadius::same(Radius::MD))
        .inner_margin(egui::Margin::symmetric(Space::SM, Space::TWO_XS))
        .show(ui, |ui| {
            let response =
                ui.allocate_response(egui::vec2(ui.available_width(), 36.0), egui::Sense::click());
            response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::SelectableLabel,
                    ui.is_enabled(),
                    a11y.launcher_result_label(
                        &result.action.name,
                        action_kind(&result.action.action_type),
                        index + 1,
                        total,
                    ),
                )
            });
            let rect = response.rect.shrink2(egui::vec2(Space::XS as f32, 0.0));
            ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                ui.horizontal(|ui| {
                    render_kind_badge(ui, &ctx, &result.action.action_type);
                    ui.label(
                        egui::RichText::new(&result.action.name)
                            .font(Text::body())
                            .color(Color::fg_primary(&ctx))
                            .strong(),
                    );
                    ui.label(
                        egui::RichText::new(action_kind(&result.action.action_type))
                            .font(Text::footnote())
                            .color(Color::fg_secondary(&ctx)),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if selected {
                            keycap(ui, "Enter");
                            quiet_label(ui, i18n::t("launcher.action.run"));
                        } else if index < 9 {
                            keycap(ui, &format!("Mod+{}", index + 1));
                        }
                    });
                });
            });
            response
        })
        .inner
}

fn draw_focus_ring(ui: &egui::Ui, rect: egui::Rect, radius: u8, width: f32) {
    let outer = rect.expand(3.0);
    ui.painter().rect_stroke(
        outer,
        egui::CornerRadius::same(radius),
        egui::Stroke::new(width, Color::accent_base(ui.ctx())),
        egui::StrokeKind::Outside,
    );
}

fn render_action_bar(
    ui: &mut egui::Ui,
    state: &LauncherState,
    hotkey_status: &str,
    resident_status: &str,
) -> egui::Rect {
    let ctx = ui.ctx().clone();
    egui::Frame::new()
        .fill(Color::bg_surface_1(&ctx))
        .corner_radius(egui::CornerRadius::same(Radius::MD))
        .inner_margin(egui::Margin::symmetric(Space::XS, Space::TWO_XS))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let right_width = 272.0_f32.min(ui.available_width() * 0.48);
                let left_width = (ui.available_width() - right_width - Space::XS as f32).max(160.0);
                ui.allocate_ui_with_layout(
                    egui::vec2(left_width, 24.0),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| render_action_summary(ui, state, left_width),
                );
                ui.allocate_ui_with_layout(
                    egui::vec2(right_width, 24.0),
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        keycap(ui, &input::launcher_action_panel().label());
                        quiet_label(ui, i18n::t("launcher.action.actions"));
                        keycap(ui, "Enter");
                        quiet_label(ui, i18n::t("launcher.action.run"));
                        quiet_label(ui, hotkey_status);
                        ui.label(
                            egui::RichText::new(resident_status)
                                .font(Text::caption())
                                .color(Color::fg_secondary(&ctx)),
                        );
                    },
                );
            });
        })
        .response
        .rect
}

fn render_action_summary(ui: &mut egui::Ui, state: &LauncherState, max_width: f32) {
    let ctx = ui.ctx().clone();
    if let Some(preview) = state.view.preview.as_ref() {
        ui.add_sized(
            [max_width * 0.34, 18.0],
            egui::Label::new(
                egui::RichText::new(&preview.title)
                    .font(Text::footnote())
                    .color(Color::fg_primary(&ctx))
                    .strong(),
            )
            .truncate(),
        );
        ui.add_sized(
            [max_width * 0.62, 18.0],
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
        [max_width, 18.0],
        egui::Label::new(
            egui::RichText::new("Press / for commands")
                .font(Text::footnote())
                .color(Color::fg_secondary(&ctx)),
        )
        .truncate(),
    );
}

fn render_voice(ui: &mut egui::Ui, state: &mut LauncherState, voice_transcript: &mut String) {
    if !state.controller.voice_active {
        return;
    }
    let ctx = ui.ctx().clone();
    ui.add_space(Space::XS as f32);
    egui::Frame::new()
        .fill(Color::bg_surface_2(&ctx))
        .corner_radius(egui::CornerRadius::same(Radius::MD))
        .inner_margin(egui::Margin::same(Space::XS))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Voice").color(Color::fg_secondary(&ctx)));
                ui.add_sized(
                    [ui.available_width() - 112.0, 28.0],
                    egui::TextEdit::singleline(voice_transcript).hint_text("voice transcript"),
                );
                if quiet_button(ui, "Apply").clicked() {
                    state.apply_voice_transcript(voice_transcript.as_str());
                }
            });
        });
}

fn render_feedback(ui: &mut egui::Ui, state: &LauncherState) {
    let ctx = ui.ctx().clone();
    let feedback = state.view.feedback.as_ref();
    if feedback.is_none() {
        return;
    }
    egui::Frame::new()
        .fill(feedback_fill(&ctx, feedback))
        .stroke(egui::Stroke::new(1.0, feedback_stroke(&ctx, feedback)))
        .corner_radius(egui::CornerRadius::same(Radius::MD))
        .inner_margin(egui::Margin::symmetric(Space::SM, Space::XS))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                if let Some(feedback) = feedback {
                    ui.label(
                        egui::RichText::new(feedback_marker(feedback))
                            .font(Text::body())
                            .color(feedback_stroke(&ctx, Some(feedback))),
                    );
                    ui.label(
                        egui::RichText::new(&feedback.title)
                            .font(Text::body())
                            .color(Color::fg_primary(&ctx))
                            .strong(),
                    );
                    ui.label(
                        egui::RichText::new(&feedback.action_name)
                            .font(Text::footnote())
                            .color(Color::fg_primary(&ctx)),
                    );
                    ui.label(
                        egui::RichText::new(&feedback.detail)
                            .font(Text::footnote())
                            .color(Color::fg_secondary(&ctx)),
                    );
                }
                render_performance(ui, &state.performance_report());
            });
        });
}

fn group_header(ui: &mut egui::Ui, group: &str) {
    let ctx = ui.ctx().clone();
    ui.add_space(Space::XS as f32);
    ui.label(
        egui::RichText::new(group)
            .font(Text::footnote())
            .color(Color::fg_tertiary(&ctx))
            .strong(),
    );
}

fn section_header(ui: &mut egui::Ui, title: &str, detail: &str) {
    let ctx = ui.ctx().clone();
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(title)
                .font(Text::body())
                .color(Color::fg_primary(&ctx))
                .strong(),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new(detail).font(Text::footnote()));
        });
    });
    ui.add_space(Space::TWO_XS as f32);
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

fn action_group(result: &SearchResult) -> String {
    match &result.action.action_type {
        ActionType::AppLaunch => "App / File".to_string(),
        ActionType::Workflow => "Action / Workflow".to_string(),
        ActionType::Command => "Action / Workflow".to_string(),
        ActionType::Skill => "Memory / Skill".to_string(),
        ActionType::Clipboard => "Clipboard".to_string(),
        ActionType::Custom(kind) if kind == "file" => "App / File".to_string(),
        ActionType::Custom(_) => "Other".to_string(),
    }
}

fn action_kind(action_type: &ActionType) -> &str {
    match action_type {
        ActionType::AppLaunch => "App",
        ActionType::Workflow => "Workflow",
        ActionType::Command => "Command",
        ActionType::Skill => "Skill",
        ActionType::Clipboard => "Clipboard",
        ActionType::Custom(kind) if kind == "file" => "File",
        ActionType::Custom(_) => "Custom",
    }
}

fn render_kind_badge(ui: &mut egui::Ui, ctx: &egui::Context, action_type: &ActionType) {
    egui::Frame::new()
        .fill(Color::bg_surface_2(ctx))
        .corner_radius(egui::CornerRadius::same(Radius::SM))
        .inner_margin(egui::Margin::symmetric(Space::XS, Space::TWO_XS))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(action_kind(action_type))
                    .font(Text::caption())
                    .color(Color::fg_secondary(ctx)),
            );
        });
}
