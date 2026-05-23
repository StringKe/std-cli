use crate::{
    ui,
    workspace_panes::{focused_workspace_spec, StudioWorkspaceSpec},
    StudioEguiApp,
};
use eframe::egui;
use std_egui::{
    input,
    tokens::{Color, Elevation, OverlaySize, Radius, Space, Text},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StudioContextHelp {
    pub title: String,
    pub detail: String,
    pub signals: Vec<String>,
    pub shortcuts: Vec<ContextHelpShortcut>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ContextHelpShortcut {
    pub label: String,
    pub action: String,
}

impl StudioContextHelp {
    pub(crate) fn from_app(app: &std_studio::StudioApp) -> Self {
        focused_workspace_spec(app)
            .map(|spec| context_help_for_spec(&spec))
            .unwrap_or_else(context_help_for_dashboard_workspace)
    }

    #[cfg(test)]
    pub(crate) fn summary(&self) -> String {
        format!(
            "context_help=title:{};detail:{};signals={};shortcuts={}",
            self.title,
            self.detail,
            self.signals.len(),
            self.shortcuts.len()
        )
    }
}

impl StudioEguiApp {
    pub(crate) fn render_context_help_overlay(&mut self, ctx: &egui::Context) {
        if !self.layout.context_help_open {
            return;
        }
        if std_egui::input::ime_action_guard(ctx).blocks_actions() {
            return;
        }
        if input::escape().pressed(ctx) {
            self.layout.close_overlays();
            return;
        }
        let help = StudioContextHelp::from_app(&self.app);
        egui::Area::new(egui::Id::new("studio_context_help"))
            .anchor(
                egui::Align2::CENTER_TOP,
                OverlaySize::context_help_anchor_offset(),
            )
            .order(egui::Order::Foreground)
            .constrain(true)
            .show(ctx, |ui| {
                render_context_help_frame(ui, &help);
            });
    }
}

fn context_help_for_spec(spec: &StudioWorkspaceSpec) -> StudioContextHelp {
    let mut signals = vec![format!("kind {}", spec.content_key)];
    signals.extend(spec.lines.iter().take(3).cloned());
    if let Some(path) = &spec.workflow_path {
        signals.push(format!("workflow {}", path.display()));
    }
    if let Some(path) = &spec.analysis_path {
        signals.push(format!("analysis {}", path.display()));
    }
    StudioContextHelp {
        title: spec.title.clone(),
        detail: format!("{} workspace pane", spec.content_key),
        signals,
        shortcuts: studio_shortcuts(),
    }
}

fn context_help_for_dashboard_workspace() -> StudioContextHelp {
    let pane = std_studio::StudioPane::Dashboard;
    StudioContextHelp {
        title: pane.label().to_string(),
        detail: format!("{} workspace pane", pane.content_key()),
        signals: vec![
            format!("content {}", pane.content_key()),
            "internal workspace panes stay inside the Studio host".to_string(),
        ],
        shortcuts: studio_shortcuts(),
    }
}

fn studio_shortcuts() -> Vec<ContextHelpShortcut> {
    vec![
        shortcut(input::studio_context_help(), "Open context help"),
        shortcut(
            input::studio_command_palette_slash(),
            "Open command palette",
        ),
        shortcut(input::studio_command_palette(), "Open command palette"),
        shortcut(input::studio_quick_open(), "Open workspace pane"),
        shortcut(input::studio_new_workflow(), "Create workflow"),
        shortcut(input::studio_bottom_panel_toggle(), "Toggle bottom panel"),
        shortcut(input::studio_inspector_toggle(), "Toggle inspector"),
        shortcut(input::studio_zoom_in(), "Increase UI scale"),
        shortcut(input::studio_zoom_out(), "Decrease UI scale"),
        shortcut(input::studio_zoom_reset(), "Reset UI scale"),
        shortcut(input::escape(), "Close overlay"),
    ]
}

fn shortcut(binding: input::KeyBinding, action: &str) -> ContextHelpShortcut {
    ContextHelpShortcut {
        label: binding.label(),
        action: action.to_string(),
    }
}

fn render_context_help_frame(ui: &mut egui::Ui, help: &StudioContextHelp) {
    let ctx = ui.ctx().clone();
    egui::Frame::new()
        .fill(Color::bg_surface_1(&ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_border(&ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::MD))
        .shadow(Elevation::level_2(&ctx))
        .inner_margin(egui::Margin::same(Space::MD))
        .show(ui, |ui| {
            ui.set_width(OverlaySize::context_help_width());
            ui::section_header(ui, &help.title, &help.detail);
            render_signals(ui, &help.signals);
            ui.add_space(Space::SM as f32);
            render_shortcuts(ui, &help.shortcuts);
        })
        .response
        .widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Label,
                ui.is_enabled(),
                context_help_a11y_label(help),
            )
        });
}

fn render_signals(ui: &mut egui::Ui, signals: &[String]) {
    for signal in signals {
        ui.label(egui::RichText::new(signal).color(ui::muted_text(ui.ctx())));
    }
}

fn render_shortcuts(ui: &mut egui::Ui, shortcuts: &[ContextHelpShortcut]) {
    egui::Grid::new("studio_context_help_shortcuts")
        .num_columns(2)
        .spacing(OverlaySize::context_help_grid_spacing())
        .striped(false)
        .show(ui, |ui| {
            for shortcut in shortcuts {
                ui.label(
                    egui::RichText::new(&shortcut.label)
                        .font(Text::code())
                        .color(ui::strong_text(ui.ctx())),
                );
                ui.label(egui::RichText::new(&shortcut.action).color(ui::muted_text(ui.ctx())));
                ui.end_row();
            }
        });
}

fn context_help_a11y_label(help: &StudioContextHelp) -> String {
    format!(
        "Context help, {}, {}, {} shortcuts",
        help.title,
        help.detail,
        help.shortcuts.len()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_help_uses_internal_overlay_not_native_window() {
        let source = include_str!("context_help.rs");
        let implementation = source.split("mod tests").next().unwrap();

        assert!(implementation.contains("egui::Area"));
        assert!(!implementation.contains("egui::Window"));
        assert!(implementation.contains("Color::bg_surface_1"));
        assert!(implementation.contains("OverlaySize::context_help_anchor_offset()"));
        assert!(implementation.contains("OverlaySize::context_help_width()"));
        assert!(implementation.contains("context_help_a11y_label"));
    }

    #[test]
    fn context_help_summary_exposes_current_workspace() {
        let mut app = std_studio::StudioApp::default();
        app.open_plugin_manager_pane();

        let help = StudioContextHelp::from_app(&app);

        assert!(help.summary().contains("context_help=title:插件管理"));
        assert!(help.summary().contains("detail:plugins workspace pane"));
        assert!(help.shortcuts.iter().any(|item| item.label == "F1"));
    }

    #[test]
    fn context_help_a11y_label_exposes_scope_and_shortcut_count() {
        let help = StudioContextHelp {
            title: "Dashboard".to_string(),
            detail: "dashboard workspace pane".to_string(),
            signals: vec!["content dashboard".to_string()],
            shortcuts: vec![shortcut(input::studio_context_help(), "Open context help")],
        };

        assert_eq!(
            context_help_a11y_label(&help),
            "Context help, Dashboard, dashboard workspace pane, 1 shortcuts"
        );
    }
}
