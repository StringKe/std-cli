use crate::{
    ui,
    views::plugin_rows::{label_value_row, status_row},
};
use eframe::egui;
use std_egui::tokens::Text;
use std_studio::plugin_security::runtime_summary;
use std_types::ActionExecutionStatus;

pub(crate) fn execution_panel(
    ui: &mut egui::Ui,
    name: &str,
    status: &ActionExecutionStatus,
    message: &str,
    output: Option<&serde_json::Value>,
) {
    let runtime = runtime_summary(status, output);
    status_row(
        ui,
        name,
        &runtime.status,
        message,
        plugin_status_fill(ui.ctx(), status),
    );
    label_value_row(ui, "runtime", &runtime.runtime);
    label_value_row(ui, "exit", &runtime.exit_code);
    label_value_row(ui, "duration", &runtime.duration);
    label_value_row(ui, "boundary", &runtime.boundary);
}

pub(crate) fn output_view(ui: &mut egui::Ui, output: &serde_json::Value) {
    let body = output.to_string();
    let response = ui.add_sized(
        [ui.available_width(), 120.0],
        egui::Label::new(
            egui::RichText::new(body)
                .font(Text::code())
                .color(ui::strong_text(ui.ctx())),
        )
        .selectable(true),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            "Plugin runtime output",
        )
    });
}

fn plugin_status_fill(ctx: &egui::Context, status: &ActionExecutionStatus) -> egui::Color32 {
    match status {
        ActionExecutionStatus::Completed => ui::ok_bg(ctx),
        ActionExecutionStatus::Failed => ui::danger_bg(ctx),
        ActionExecutionStatus::NeedsExternalRunner => ui::warn_bg(ctx),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plugin_runtime_output_uses_readonly_label_not_text_edit() {
        let source = include_str!("plugin_runtime_rows.rs");
        let implementation = source.split("#[cfg(test)]").next().unwrap();

        assert!(implementation.contains("fn output_view"));
        assert!(implementation.contains("WidgetType::Label"));
        assert!(implementation.contains("Plugin runtime output"));
        assert!(implementation.contains(".selectable(true)"));
        assert!(!implementation.contains("TextEdit::multiline(&mut body).interactive(false)"));
    }

    #[test]
    fn plugin_runtime_status_fills_distinguish_failed_from_external_runner() {
        let ctx = egui::Context::default();
        std_egui::tokens::apply_theme(&ctx, std_egui::tokens::ThemeMode::Light);

        assert_eq!(
            plugin_status_fill(&ctx, &ActionExecutionStatus::Failed),
            ui::danger_bg(&ctx)
        );
        assert_eq!(
            plugin_status_fill(&ctx, &ActionExecutionStatus::NeedsExternalRunner),
            ui::warn_bg(&ctx)
        );
    }
}
