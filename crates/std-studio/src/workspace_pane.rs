use crate::{StudioApp, StudioPane};
use serde::{Deserialize, Serialize};
use std_egui::i18n;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkspacePaneId(u64);

impl WorkspacePaneId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspacePaneKind {
    Pane(StudioPane),
    WorkflowBuilder { workflow_path: std::path::PathBuf },
    AnalysisWorkbench { entity_path: std::path::PathBuf },
    AppManager,
    MemoryBrowser,
    ExecutionHistory,
    PluginManager,
    Settings,
}

impl WorkspacePaneKind {
    pub fn closable(&self) -> bool {
        !matches!(self, WorkspacePaneKind::Pane(StudioPane::Dashboard))
    }

    pub fn identity_key(&self) -> String {
        match self {
            WorkspacePaneKind::Pane(pane) => format!("pane:{}", pane.content_key()),
            WorkspacePaneKind::WorkflowBuilder { workflow_path } => {
                format!("workflow:{}", lexical_path_key(workflow_path))
            }
            WorkspacePaneKind::AnalysisWorkbench { entity_path } => {
                format!("analysis:{}", lexical_path_key(entity_path))
            }
            WorkspacePaneKind::AppManager => "singleton:apps".to_string(),
            WorkspacePaneKind::MemoryBrowser => "singleton:memory".to_string(),
            WorkspacePaneKind::ExecutionHistory => "singleton:history".to_string(),
            WorkspacePaneKind::PluginManager => "singleton:plugins".to_string(),
            WorkspacePaneKind::Settings => "singleton:settings".to_string(),
        }
    }

    pub fn title(&self) -> String {
        match self {
            WorkspacePaneKind::Pane(pane) => pane.label().to_string(),
            WorkspacePaneKind::WorkflowBuilder { workflow_path } => {
                format!(
                    "{}: {}",
                    i18n::t("studio.shell.open.workflow.title"),
                    display_name(workflow_path)
                )
            }
            WorkspacePaneKind::AnalysisWorkbench { entity_path } => {
                format!(
                    "{}: {}",
                    i18n::t("studio.shell.open.analysis.title"),
                    display_name(entity_path)
                )
            }
            WorkspacePaneKind::AppManager => {
                i18n::t("studio.workspace_panes.app_manager").to_string()
            }
            WorkspacePaneKind::MemoryBrowser => {
                i18n::t("studio.workspace_panes.memory_browser").to_string()
            }
            WorkspacePaneKind::ExecutionHistory => {
                i18n::t("studio.workspace_panes.execution_history").to_string()
            }
            WorkspacePaneKind::PluginManager => {
                i18n::t("studio.workspace_panes.plugin_manager").to_string()
            }
            WorkspacePaneKind::Settings => i18n::t("studio.settings.title").to_string(),
        }
    }

    pub fn content_key(&self) -> &'static str {
        match self {
            WorkspacePaneKind::Pane(pane) => pane.content_key(),
            WorkspacePaneKind::WorkflowBuilder { .. } => "workflows",
            WorkspacePaneKind::AnalysisWorkbench { .. } => "analysis",
            WorkspacePaneKind::AppManager => "apps",
            WorkspacePaneKind::MemoryBrowser => "memory",
            WorkspacePaneKind::ExecutionHistory => "history",
            WorkspacePaneKind::PluginManager => "plugins",
            WorkspacePaneKind::Settings => "settings",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspacePane {
    pub id: WorkspacePaneId,
    pub kind: WorkspacePaneKind,
    pub title: String,
    pub open: bool,
    pub focused_at: chrono::DateTime<chrono::Utc>,
    pub focus_serial: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspacePaneContent {
    pub content_key: &'static str,
    pub heading: String,
    pub lines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspacePaneCloseSnapshot {
    pub id: WorkspacePaneId,
    pub kind: WorkspacePaneKind,
    pub identity_key: String,
    pub content_key: String,
    pub title: String,
    pub focused: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspacePaneCloseGuard {
    pub panes: Vec<WorkspacePaneCloseSnapshot>,
    pub focused_pane: Option<WorkspacePaneId>,
}

impl WorkspacePane {
    pub fn new(id: WorkspacePaneId, kind: WorkspacePaneKind, focus_serial: u64) -> Self {
        let title = kind.title();
        Self {
            id,
            kind,
            title,
            open: true,
            focused_at: chrono::Utc::now(),
            focus_serial,
        }
    }

    pub fn focus(&mut self, serial: u64) {
        self.focused_at = chrono::Utc::now();
        self.focus_serial = serial;
    }
}

impl StudioApp {
    pub fn open_workspace_pane(&mut self, pane: StudioPane) -> WorkspacePaneId {
        self.open_pane(WorkspacePaneKind::Pane(pane))
    }

    pub fn open_workflow_builder(&mut self, workflow_path: std::path::PathBuf) -> WorkspacePaneId {
        self.open_pane(WorkspacePaneKind::WorkflowBuilder { workflow_path })
    }

    pub fn open_analysis_workbench(&mut self, entity_path: std::path::PathBuf) -> WorkspacePaneId {
        self.open_pane(WorkspacePaneKind::AnalysisWorkbench { entity_path })
    }

    pub fn open_plugin_manager_pane(&mut self) -> WorkspacePaneId {
        self.open_pane(WorkspacePaneKind::PluginManager)
    }

    pub fn open_app_manager_pane(&mut self) -> WorkspacePaneId {
        self.open_pane(WorkspacePaneKind::AppManager)
    }

    pub fn open_memory_browser_pane(&mut self) -> WorkspacePaneId {
        self.open_pane(WorkspacePaneKind::MemoryBrowser)
    }

    pub fn open_execution_history_pane(&mut self) -> WorkspacePaneId {
        self.open_pane(WorkspacePaneKind::ExecutionHistory)
    }

    pub fn open_settings_pane(&mut self) -> WorkspacePaneId {
        self.open_pane(WorkspacePaneKind::Settings)
    }

    pub fn focused_studio_pane(&self) -> Option<StudioPane> {
        let focused = self.focused_pane?;
        let pane = self
            .workspace_panes
            .iter()
            .find(|pane| pane.id == focused && pane.open)?;
        Some(match pane.kind {
            WorkspacePaneKind::Pane(pane) => pane,
            WorkspacePaneKind::WorkflowBuilder { .. } => StudioPane::Workflows,
            WorkspacePaneKind::AnalysisWorkbench { .. } => StudioPane::Analysis,
            WorkspacePaneKind::AppManager => StudioPane::Apps,
            WorkspacePaneKind::MemoryBrowser => StudioPane::Memory,
            WorkspacePaneKind::ExecutionHistory => StudioPane::History,
            WorkspacePaneKind::PluginManager => StudioPane::Plugins,
            WorkspacePaneKind::Settings => StudioPane::Settings,
        })
    }

    pub fn focus_workspace_pane(&mut self, id: WorkspacePaneId) -> bool {
        let Some(index) = self.workspace_panes.iter().position(|pane| pane.id == id) else {
            return false;
        };
        let serial = self.next_focus_serial();
        let pane = &mut self.workspace_panes[index];
        pane.open = true;
        pane.focus(serial);
        self.focused_pane = Some(id);
        true
    }

    pub fn focus_next_workspace_pane(&mut self) -> Option<WorkspacePaneId> {
        self.focus_workspace_pane_by_offset(1)
    }

    pub fn focus_previous_workspace_pane(&mut self) -> Option<WorkspacePaneId> {
        self.focus_workspace_pane_by_offset(-1)
    }

    pub fn close_workspace_pane(&mut self, id: WorkspacePaneId) -> bool {
        let Some(index) = self.workspace_panes.iter().position(|pane| pane.id == id) else {
            return false;
        };
        if !self.workspace_panes[index].kind.closable() {
            return false;
        }
        if !self.workspace_panes[index].open {
            return false;
        }
        self.workspace_panes[index].open = false;
        if self.focused_pane == Some(id) {
            self.focused_pane = self
                .workspace_panes
                .iter()
                .filter(|pane| pane.open)
                .max_by_key(|pane| pane.focus_serial)
                .map(|pane| pane.id);
        }
        true
    }

    pub fn open_workspace_panes(&self) -> impl Iterator<Item = &WorkspacePane> {
        self.workspace_panes.iter().filter(|pane| pane.open)
    }

    pub fn prepare_workspace_closeguard(&self) -> WorkspacePaneCloseGuard {
        WorkspacePaneCloseGuard {
            panes: self
                .open_workspace_panes()
                .map(|pane| WorkspacePaneCloseSnapshot {
                    id: pane.id,
                    kind: pane.kind.clone(),
                    identity_key: pane.kind.identity_key(),
                    content_key: pane.kind.content_key().to_string(),
                    title: pane.title.clone(),
                    focused: self.focused_pane == Some(pane.id),
                })
                .collect(),
            focused_pane: self.focused_pane,
        }
    }

    pub fn close_workspace_instance(&mut self) -> WorkspacePaneCloseGuard {
        let closeguard = self.prepare_workspace_closeguard();
        for pane in &mut self.workspace_panes {
            pane.open = false;
        }
        self.focused_pane = None;
        closeguard
    }

    pub fn restore_workspace_closeguard(&mut self, closeguard: &WorkspacePaneCloseGuard) {
        for snapshot in &closeguard.panes {
            self.next_pane_serial = self.next_pane_serial.max(snapshot.id.value() + 1);
            if let Some(index) = self
                .workspace_panes
                .iter()
                .position(|pane| pane.id == snapshot.id)
            {
                let focus_serial = self.next_focus_serial();
                let pane = &mut self.workspace_panes[index];
                pane.kind = snapshot.kind.clone();
                pane.title = snapshot.title.clone();
                pane.open = true;
                pane.focus(focus_serial);
            } else {
                let focus_serial = self.next_focus_serial();
                self.workspace_panes.push(WorkspacePane {
                    id: snapshot.id,
                    kind: snapshot.kind.clone(),
                    title: snapshot.title.clone(),
                    open: true,
                    focused_at: chrono::Utc::now(),
                    focus_serial,
                });
            }
        }
        self.focused_pane = closeguard.focused_pane.filter(|focused| {
            self.workspace_panes
                .iter()
                .any(|pane| pane.id == *focused && pane.open)
        });
    }

    fn focus_workspace_pane_by_offset(&mut self, offset: isize) -> Option<WorkspacePaneId> {
        let open_ids = self
            .workspace_panes
            .iter()
            .filter(|pane| pane.open)
            .map(|pane| pane.id)
            .collect::<Vec<_>>();
        if open_ids.is_empty() {
            self.focused_pane = None;
            return None;
        }
        let current = self
            .focused_pane
            .and_then(|id| open_ids.iter().position(|open_id| *open_id == id))
            .unwrap_or(0);
        let next = (current as isize + offset).rem_euclid(open_ids.len() as isize) as usize;
        let id = open_ids[next];
        self.focus_workspace_pane(id).then_some(id)
    }

    fn open_pane(&mut self, kind: WorkspacePaneKind) -> WorkspacePaneId {
        let identity_key = kind.identity_key();
        if let Some(index) = self
            .workspace_panes
            .iter()
            .position(|pane| pane.kind.identity_key() == identity_key)
        {
            let serial = self.next_focus_serial();
            let existing = &mut self.workspace_panes[index];
            existing.open = true;
            existing.focus(serial);
            self.focused_pane = Some(existing.id);
            return existing.id;
        }

        let id = WorkspacePaneId::new(self.next_pane_serial);
        self.next_pane_serial += 1;
        let focus_serial = self.next_focus_serial();
        let pane = WorkspacePane::new(id, kind, focus_serial);
        self.focused_pane = Some(id);
        self.workspace_panes.push(pane);
        id
    }

    fn next_focus_serial(&mut self) -> u64 {
        let serial = self.next_focus_serial;
        self.next_focus_serial += 1;
        serial
    }
}

fn display_name(path: &std::path::Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(ToString::to_string)
        .unwrap_or_else(|| path.display().to_string())
}

fn lexical_path_key(path: &std::path::Path) -> String {
    let mut parts = Vec::new();
    let mut prefix = String::new();
    for component in path.components() {
        match component {
            std::path::Component::Prefix(value) => {
                prefix = value.as_os_str().to_string_lossy().into()
            }
            std::path::Component::RootDir => prefix.push('/'),
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                if parts.last().is_some_and(|part| part != "..") {
                    parts.pop();
                } else {
                    parts.push("..".to_string());
                }
            }
            std::path::Component::Normal(value) => parts.push(value.to_string_lossy().into()),
        }
    }
    if parts.is_empty() {
        return if prefix.is_empty() {
            ".".to_string()
        } else {
            prefix
        };
    }
    if prefix.is_empty() || prefix == "/" {
        format!("{prefix}{}", parts.join("/"))
    } else {
        format!("{prefix}/{}", parts.join("/"))
    }
}
