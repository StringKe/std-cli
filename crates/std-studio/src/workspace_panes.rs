use crate::{ui, StudioEguiApp};
use eframe::egui;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std_egui::{i18n, input, tokens::Space};
use std_studio::{StudioPane, WorkspacePane, WorkspacePaneId, WorkspacePaneKind};

pub(crate) type WorkspaceCommandQueue = Arc<Mutex<Vec<StudioWorkspaceCommand>>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StudioWorkspaceCommand {
    Focus(WorkspacePaneId),
    FocusNext,
    FocusPrevious,
    Close(WorkspacePaneId),
    ShowInMain(StudioPane),
    Refresh,
    PreviewWorkflow(PathBuf),
    RunWorkflow(PathBuf),
    Analyze(PathBuf),
    ReloadPlugins,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StudioWorkspaceSpec {
    pub id: WorkspacePaneId,
    pub title: String,
    pub content_key: &'static str,
    pub heading: String,
    pub lines: Vec<String>,
    pub pane: StudioPane,
    pub workflow_path: Option<PathBuf>,
    pub analysis_path: Option<PathBuf>,
}

impl StudioWorkspaceSpec {
    fn from_pane(app: &std_studio::StudioApp, pane: &WorkspacePane) -> Self {
        let content = app.workspace_pane_content(&pane.kind);
        let (workflow_path, analysis_path) = match &pane.kind {
            WorkspacePaneKind::WorkflowBuilder { workflow_path } => {
                (Some(workflow_path.clone()), None)
            }
            WorkspacePaneKind::AnalysisWorkbench { entity_path } => {
                (None, Some(entity_path.clone()))
            }
            _ => (None, None),
        };
        let main_pane = match &pane.kind {
            WorkspacePaneKind::Pane(pane) => *pane,
            WorkspacePaneKind::WorkflowBuilder { .. } => StudioPane::Workflows,
            WorkspacePaneKind::AnalysisWorkbench { .. } => StudioPane::Analysis,
            WorkspacePaneKind::AppManager => StudioPane::Apps,
            WorkspacePaneKind::MemoryBrowser => StudioPane::Memory,
            WorkspacePaneKind::ExecutionHistory => StudioPane::History,
            WorkspacePaneKind::PluginManager => StudioPane::Plugins,
            WorkspacePaneKind::Settings => StudioPane::Settings,
        };
        Self {
            id: pane.id,
            title: pane.title.clone(),
            content_key: pane.kind.content_key(),
            heading: content.heading,
            lines: content.lines,
            pane: main_pane,
            workflow_path,
            analysis_path,
        }
    }
}

pub(crate) fn focused_workspace_spec(app: &std_studio::StudioApp) -> Option<StudioWorkspaceSpec> {
    let focused = app.focused_pane?;
    app.open_workspace_panes()
        .find(|pane| pane.id == focused)
        .map(|pane| StudioWorkspaceSpec::from_pane(app, pane))
        .or_else(|| {
            app.open_workspace_panes()
                .last()
                .map(|pane| StudioWorkspaceSpec::from_pane(app, pane))
        })
}

impl StudioEguiApp {
    pub(crate) fn render_focused_workspace_pane(&mut self, ui: &mut egui::Ui) -> bool {
        let Some(spec) = focused_workspace_spec(&self.app) else {
            return false;
        };

        let tabs = crate::workspace_tabs::workspace_tab_specs(
            &self.app.workspace_panes,
            self.app.focused_pane,
        );
        crate::workspace_tabs::render_workspace_tabs(ui, &tabs, &self.workspace_commands);
        ui.add_space(Space::XS as f32);
        render_workspace_toolbar(
            ui,
            &spec,
            &self.workspace_commands,
            &mut self.pending_workspace_focus,
        );
        ui.add_space(Space::XS as f32);
        crate::workspace_pane_content::render_workspace_content(self, ui, &spec);
        true
    }

    pub(crate) fn consume_workspace_commands(&mut self) {
        let commands = self
            .workspace_commands
            .lock()
            .map(|mut queue| queue.drain(..).collect::<Vec<_>>())
            .unwrap_or_default();

        for command in commands {
            self.apply_workspace_command(command);
        }
    }

    fn apply_workspace_command(&mut self, command: StudioWorkspaceCommand) {
        match command {
            StudioWorkspaceCommand::Focus(id) => {
                if self.app.focus_workspace_pane(id) {
                    self.pending_workspace_focus = Some(id);
                    self.status = format!("focused workspace pane {}", id.value());
                }
            }
            StudioWorkspaceCommand::FocusNext => {
                if let Some(id) = self.app.focus_next_workspace_pane() {
                    self.pending_workspace_focus = Some(id);
                    self.status = format!("focused workspace pane {}", id.value());
                }
            }
            StudioWorkspaceCommand::FocusPrevious => {
                if let Some(id) = self.app.focus_previous_workspace_pane() {
                    self.pending_workspace_focus = Some(id);
                    self.status = format!("focused workspace pane {}", id.value());
                }
            }
            StudioWorkspaceCommand::Close(id) => {
                if self.app.close_workspace_pane(id) {
                    self.pending_workspace_focus = self.app.focused_pane;
                    self.status = format!("closed workspace pane {}", id.value());
                }
            }
            StudioWorkspaceCommand::ShowInMain(pane) => {
                self.app.switch_pane(pane);
                self.status = format!("showing {} in main workspace", pane.label());
            }
            StudioWorkspaceCommand::Refresh => {
                self.app.refresh();
                self.status = "refreshed workspace state".to_string();
            }
            StudioWorkspaceCommand::PreviewWorkflow(path) => {
                match self.app.preview_workflow_path(&path) {
                    Ok(report) => {
                        let status = format!("workspace preview {:?}", report.status);
                        self.open_batch_debug_panel();
                        self.status = status;
                    }
                    Err(error) => self.status = error.to_string(),
                }
            }
            StudioWorkspaceCommand::RunWorkflow(path) => match self.app.run_workflow_path(&path) {
                Ok(execution) => {
                    let status = format!("workspace run {:?}", execution.status);
                    self.open_batch_debug_panel();
                    self.status = status;
                }
                Err(error) => self.status = error.to_string(),
            },
            StudioWorkspaceCommand::Analyze(path) => match self.app.analyze_entity(&path) {
                Ok(document) => {
                    self.analysis.path = path.display().to_string();
                    self.status = format!(
                        "workspace analyzed {} components",
                        document.components.len()
                    );
                }
                Err(error) => self.status = error.to_string(),
            },
            StudioWorkspaceCommand::ReloadPlugins => match self.app.reload_plugins() {
                Ok(manager) => {
                    self.status = format!(
                        "workspace reloaded {} manifests",
                        manager.manifest_paths.len()
                    )
                }
                Err(error) => self.status = error.to_string(),
            },
        }
    }
}

fn render_workspace_toolbar(
    ui: &mut egui::Ui,
    spec: &StudioWorkspaceSpec,
    commands: &WorkspaceCommandQueue,
    pending_focus: &mut Option<WorkspacePaneId>,
) {
    workspace_focus_sentinel(ui, spec, pending_focus);
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new(&spec.heading).color(ui::strong_text(ui.ctx())));
        render_workspace_summary(ui, spec);
        render_workspace_actions(ui, spec, commands);
    });
}

pub(crate) fn workspace_pane_a11y_label(spec: &StudioWorkspaceSpec) -> String {
    format!("Workspace pane, {}, {}", spec.heading, spec.content_key)
}

pub(crate) fn workspace_pane_focus_id(id: WorkspacePaneId) -> egui::Id {
    egui::Id::new(("studio.workspace_pane", id.value()))
}

fn request_workspace_focus(
    ui: &mut egui::Ui,
    id: WorkspacePaneId,
    pending_focus: &mut Option<WorkspacePaneId>,
) {
    if *pending_focus != Some(id) {
        return;
    }
    ui.ctx()
        .memory_mut(|memory| memory.request_focus(workspace_pane_focus_id(id)));
    *pending_focus = None;
}

fn workspace_focus_sentinel(
    ui: &mut egui::Ui,
    spec: &StudioWorkspaceSpec,
    pending_focus: &mut Option<WorkspacePaneId>,
) {
    let response = ui.interact(
        egui::Rect::from_min_size(ui.cursor().min, egui::Vec2::ZERO),
        workspace_pane_focus_id(spec.id),
        egui::Sense::focusable_noninteractive(),
    );
    request_workspace_focus(ui, spec.id, pending_focus);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            workspace_pane_a11y_label(spec),
        )
    });
}

fn render_workspace_summary(ui: &mut egui::Ui, spec: &StudioWorkspaceSpec) {
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new(spec.content_key).color(ui::muted_text(ui.ctx())));
        if spec.workflow_path.is_some() {
            ui.label(
                egui::RichText::new(i18n::t("studio.workspace_panes.workflow_editor"))
                    .color(ui::muted_text(ui.ctx())),
            );
        }
        if spec.analysis_path.is_some() {
            ui.label(
                egui::RichText::new(i18n::t("studio.workspace_panes.analysis_target"))
                    .color(ui::muted_text(ui.ctx())),
            );
        }
    });
}

fn render_workspace_actions(
    ui: &mut egui::Ui,
    spec: &StudioWorkspaceSpec,
    commands: &WorkspaceCommandQueue,
) {
    ui.horizontal_wrapped(|ui| {
        if workspace_action_button(
            ui,
            spec,
            i18n::t("studio.workspace_panes.show_in_main"),
            None,
        )
        .clicked()
        {
            push_command(commands, StudioWorkspaceCommand::ShowInMain(spec.pane));
        }
        if let Some(path) = &spec.workflow_path {
            if workspace_action_button(
                ui,
                spec,
                i18n::t("studio.workspace_panes.preview_workflow"),
                None,
            )
            .clicked()
            {
                push_command(
                    commands,
                    StudioWorkspaceCommand::PreviewWorkflow(path.clone()),
                );
            }
            if workspace_action_button(
                ui,
                spec,
                i18n::t("studio.workspace_panes.run_workflow"),
                Some(input::studio_workflow_test().label().as_str()),
            )
            .clicked()
            {
                push_command(commands, StudioWorkspaceCommand::RunWorkflow(path.clone()));
            }
        }
        if let Some(path) = &spec.analysis_path {
            if workspace_action_button(ui, spec, i18n::t("studio.workspace_panes.analyze"), None)
                .clicked()
            {
                push_command(commands, StudioWorkspaceCommand::Analyze(path.clone()));
            }
        }
        if spec.content_key == "plugins"
            && workspace_action_button(
                ui,
                spec,
                i18n::t("studio.workspace_panes.reload_plugins"),
                None,
            )
            .clicked()
        {
            push_command(commands, StudioWorkspaceCommand::ReloadPlugins);
        }
        if workspace_action_button(ui, spec, i18n::t("studio.workspace_panes.refresh"), None)
            .clicked()
        {
            push_command(commands, StudioWorkspaceCommand::Refresh);
        }
        if workspace_action_button(
            ui,
            spec,
            i18n::t("studio.workspace_panes.close"),
            Some(input::studio_close_tab().label().as_str()),
        )
        .clicked()
        {
            push_command(commands, StudioWorkspaceCommand::Close(spec.id));
        }
    });
}

fn workspace_action_button(
    ui: &mut egui::Ui,
    spec: &StudioWorkspaceSpec,
    label: &str,
    shortcut: Option<&str>,
) -> egui::Response {
    let response = ui::quiet_button(ui, label);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            workspace_action_a11y_label(label, spec, shortcut),
        )
    });
    response
}

pub(crate) fn workspace_action_a11y_label(
    label: &str,
    spec: &StudioWorkspaceSpec,
    shortcut: Option<&str>,
) -> String {
    let suffix = shortcut
        .map(|value| format!(", shortcut {value}"))
        .unwrap_or_default();
    format!(
        "{label}, workspace pane action, {}, button, press Enter{suffix}",
        spec.title
    )
}

fn push_command(commands: &WorkspaceCommandQueue, command: StudioWorkspaceCommand) {
    if let Ok(mut queue) = commands.lock() {
        queue.push(command);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn workspace_pane_toolbar_has_real_focus_sentinel() {
        let source = include_str!("workspace_panes.rs");
        let sentinel = source.find("fn workspace_focus_sentinel").unwrap();
        let interact = source[sentinel..].find("ui.interact(").unwrap();
        let focusable = source[sentinel..]
            .find("egui::Sense::focusable_noninteractive()")
            .unwrap();
        let id = source[sentinel..]
            .find("workspace_pane_focus_id(spec.id)")
            .unwrap();
        let request = source[sentinel..].find("request_workspace_focus(").unwrap();
        let a11y = source[sentinel..]
            .find("workspace_pane_a11y_label(spec)")
            .unwrap();

        assert!(interact < focusable);
        assert!(id < focusable);
        assert!(focusable < request);
        assert!(request < a11y);
    }
}
