use crate::{workspace_panes::StudioWorkspaceSpec, StudioEguiApp};
use eframe::egui;
use std_studio::StudioPane;

pub(crate) fn render_workspace_content(
    app: &mut StudioEguiApp,
    ui: &mut egui::Ui,
    spec: &StudioWorkspaceSpec,
) {
    match spec.content_key {
        "workflows" if spec.workflow_path.is_some() => {
            app.workflow_selected_path = spec.workflow_path.clone();
            app.render_workflow_builder(ui);
        }
        "analysis" if spec.analysis_path.is_some() => {
            if let Some(path) = &spec.analysis_path {
                app.analysis.path = path.display().to_string();
            }
            app.render_analysis(ui);
        }
        "dashboard" => app.render_dashboard(ui),
        "workflows" => app.render_workflows(ui),
        "apps" => app.render_apps(ui),
        "memory" => app.render_memory(ui),
        "plugins" => app.render_plugins(ui),
        "history" => app.render_history(ui),
        "operations" => app.render_operations(ui),
        "settings" => app.render_settings(ui),
        _ => app.render_fallback_workspace_content(ui, spec.pane),
    }
}

impl StudioEguiApp {
    fn render_fallback_workspace_content(&mut self, ui: &mut egui::Ui, pane: StudioPane) {
        match pane {
            StudioPane::Dashboard => self.render_dashboard(ui),
            StudioPane::Workflows => self.render_workflows(ui),
            StudioPane::Apps => self.render_apps(ui),
            StudioPane::Memory => self.render_memory(ui),
            StudioPane::Plugins => self.render_plugins(ui),
            StudioPane::Analysis => self.render_analysis(ui),
            StudioPane::History => self.render_history(ui),
            StudioPane::Operations => self.render_operations(ui),
            StudioPane::Settings => self.render_settings(ui),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn workspace_content_router_uses_real_studio_views() {
        let source = include_str!("workspace_pane_content.rs");

        for required in [
            "render_workflow_builder",
            "render_analysis",
            "render_plugins",
            "render_memory",
            "render_history",
            "render_operations",
            "render_settings",
            "workflow_selected_path = spec.workflow_path.clone()",
            "analysis.path = path.display().to_string()",
        ] {
            assert!(source.contains(required), "missing {required}");
        }
    }
}
