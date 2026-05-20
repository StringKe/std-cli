use crate::{
    ui_empty::{self, EmptyAction},
    ui_parts::{keycap, quiet_label, surface_frame},
};
use eframe::egui;
use std_egui::{
    a11y::AccessibilityContext,
    i18n,
    tokens::{Color, Radius, Space, Text},
    LauncherResultMode,
};
use std_launcher::LauncherState;
use std_types::{ActionType, SearchResult};

pub(crate) fn render(ui: &mut egui::Ui, state: &mut LauncherState, max_height: f32) {
    surface_frame(ui.ctx()).show(ui, |ui| {
        section_header(
            ui,
            section_title(&state.view),
            &format!("{} matches", state.view.results.len()),
        );
        render_results(ui, state, max_height);
    });
}

fn section_title(view: &std_egui::LauncherViewModel) -> &'static str {
    match view.phase {
        std_egui::LauncherPhase::Searching => "Searching",
        std_egui::LauncherPhase::Executing => "Executing",
        std_egui::LauncherPhase::Feedback => "Result",
        _ => match view.result_mode {
            LauncherResultMode::SuggestedWorkflows => "Suggested Workflows",
            LauncherResultMode::Matches => "Results",
            LauncherResultMode::NoMatches => "Results",
        },
    }
}

fn render_results(ui: &mut egui::Ui, state: &mut LauncherState, max_height: f32) {
    let mut clicked = None;
    let mut last_group = String::new();
    egui::ScrollArea::vertical()
        .id_salt("launcher_results")
        .max_height(max_height)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if state.view.results.is_empty() {
                if state.view.phase == std_egui::LauncherPhase::Searching {
                    render_progress(ui, "Searching registry and local index");
                    return;
                }
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
}

fn render_progress(ui: &mut egui::Ui, label: &str) {
    let ctx = ui.ctx().clone();
    ui.add_space(Space::MD as f32);
    ui.horizontal(|ui| {
        ui.spinner();
        ui.label(
            egui::RichText::new(label)
                .font(Text::body())
                .color(Color::fg_secondary(&ctx)),
        );
    });
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
