use eframe::egui;
use std_egui::{
    i18n,
    tokens::{Color, Radius, Space, Text},
};
use std_launcher::LauncherState;
use std_types::ActionPreview;

const MAX_EXAMPLES: usize = 2;

pub(crate) fn should_render(state: &LauncherState) -> bool {
    state.view.preview.is_some()
        && !matches!(
            state.view.phase,
            std_egui::LauncherPhase::Empty
                | std_egui::LauncherPhase::Searching
                | std_egui::LauncherPhase::Executing
        )
}

pub(crate) fn render(ui: &mut egui::Ui, state: &LauncherState) {
    let Some(preview) = state.view.preview.as_ref() else {
        return;
    };
    let ctx = ui.ctx().clone();
    egui::Frame::new()
        .fill(Color::bg_surface_2(&ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_border(&ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::md()))
        .inner_margin(egui::Margin::same(Space::sm()))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            render_header(ui, preview);
            ui.add_space(Space::two_xs() as f32);
            render_command(ui, preview);
            render_examples(ui, preview);
        })
        .response
        .widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Other,
                ui.is_enabled(),
                preview_panel_a11y_label(preview),
            )
        });
}

fn render_header(ui: &mut egui::Ui, preview: &ActionPreview) {
    let ctx = ui.ctx().clone();
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(i18n::t("launcher.preview.title"))
                .font(Text::footnote())
                .color(Color::fg_tertiary(&ctx)),
        );
        ui.label(
            egui::RichText::new(preview.title.as_str())
                .font(Text::body())
                .color(Color::fg_primary(&ctx))
                .strong(),
        );
    });
}

fn render_command(ui: &mut egui::Ui, preview: &ActionPreview) {
    let ctx = ui.ctx().clone();
    let command = if preview.primary_command.trim().is_empty() {
        preview.subtitle.as_str()
    } else {
        preview.primary_command.as_str()
    };
    egui::Frame::new()
        .fill(Color::bg_surface_0(&ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_divider(&ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::sm()))
        .inner_margin(egui::Margin::symmetric(Space::xs(), Space::two_xs()))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(command)
                    .font(Text::code())
                    .color(Color::fg_secondary(&ctx)),
            );
        });
}

fn render_examples(ui: &mut egui::Ui, preview: &ActionPreview) {
    if preview.examples.is_empty() {
        return;
    }
    let ctx = ui.ctx().clone();
    ui.add_space(Space::two_xs() as f32);
    let examples = preview
        .examples
        .iter()
        .take(MAX_EXAMPLES)
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join("  ");
    ui.label(
        egui::RichText::new(format!(
            "{} {}",
            i18n::t("launcher.preview.examples"),
            examples
        ))
        .font(Text::footnote())
        .color(Color::fg_secondary(&ctx)),
    );
}

fn preview_panel_a11y_label(preview: &ActionPreview) -> String {
    i18n::t("launcher.preview.a11y")
        .replace("{title}", &preview.title)
        .replace("{command}", &preview.primary_command)
}

#[cfg(test)]
pub(crate) fn preview_panel_contract(preview: &ActionPreview) -> String {
    format!(
        "preview_panel=visible;surface=bg/surface-2;border=stroke/border;title={};primary_command={};examples={};a11y=preview-title-command",
        !preview.title.trim().is_empty(),
        !preview.primary_command.trim().is_empty(),
        preview.examples.len().min(MAX_EXAMPLES)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_panel_contract_exposes_docs_21_preview_parts() {
        let preview = ActionPreview {
            action_id: uuid::Uuid::nil(),
            title: "Rebuild Index".to_string(),
            subtitle: "Refresh local index".to_string(),
            action_type: std_types::ActionType::Command,
            primary_command: "std index rebuild .".to_string(),
            metadata: std::collections::HashMap::new(),
            examples: vec![
                "std index rebuild .".to_string(),
                "std index status".to_string(),
                "extra hidden example".to_string(),
            ],
        };

        assert_eq!(
            preview_panel_contract(&preview),
            "preview_panel=visible;surface=bg/surface-2;border=stroke/border;title=true;primary_command=true;examples=2;a11y=preview-title-command"
        );
        assert_eq!(
            preview_panel_a11y_label(&preview),
            "预览，Rebuild Index，std index rebuild ."
        );
    }
}
