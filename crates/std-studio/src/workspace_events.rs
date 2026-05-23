use crate::{StudioApp, WorkspacePaneId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkspacePaneOperation {
    OpenNew,
    OpenExisting,
    Focus,
    Close,
    CloseBlocked,
    CloseInstance,
    Restore,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspacePaneEvent {
    pub sequence: u64,
    pub operation: WorkspacePaneOperation,
    pub pane_id: Option<WorkspacePaneId>,
    pub identity_key: String,
    pub focused_after: Option<WorkspacePaneId>,
    pub open_count: usize,
    pub native_child_windows: bool,
    pub detached_panels: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspacePaneEventTrace {
    pub events: Vec<WorkspacePaneEvent>,
}

impl WorkspacePaneOperation {
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenNew => "open-new",
            Self::OpenExisting => "open-existing",
            Self::Focus => "focus",
            Self::Close => "close",
            Self::CloseBlocked => "close-blocked",
            Self::CloseInstance => "close-instance",
            Self::Restore => "restore",
        }
    }
}

impl WorkspacePaneEventTrace {
    pub fn pass(&self) -> bool {
        !self.events.is_empty()
            && self
                .events
                .iter()
                .all(|event| !event.native_child_windows && !event.detached_panels)
            && self.has(WorkspacePaneOperation::OpenNew)
            && self.has(WorkspacePaneOperation::OpenExisting)
            && self.has(WorkspacePaneOperation::Focus)
            && self.has(WorkspacePaneOperation::Close)
            && self.has(WorkspacePaneOperation::Restore)
    }

    pub fn summary(&self) -> String {
        let path = self
            .events
            .iter()
            .map(|event| event.operation.label())
            .collect::<Vec<_>>()
            .join(">");
        let focused = self
            .events
            .last()
            .and_then(|event| event.focused_after)
            .map(|id| id.value().to_string())
            .unwrap_or_else(|| "none".to_string());
        format!(
            "event_trace={};events={};path={};focused_after={};policy=native_child_windows:false|detached_panels:false",
            self.pass(),
            self.events.len(),
            path,
            focused
        )
    }

    fn has(&self, operation: WorkspacePaneOperation) -> bool {
        self.events.iter().any(|event| event.operation == operation)
    }
}

impl StudioApp {
    pub(crate) fn record_workspace_event(
        &mut self,
        operation: WorkspacePaneOperation,
        pane_id: Option<WorkspacePaneId>,
        identity_key: impl Into<String>,
    ) {
        let event = WorkspacePaneEvent {
            sequence: self.next_workspace_event_serial,
            operation,
            pane_id,
            identity_key: identity_key.into(),
            focused_after: self.focused_pane,
            open_count: self.open_workspace_panes().count(),
            native_child_windows: self.workspace_policy.allows_native_child_windows(),
            detached_panels: self.workspace_policy.allows_detached_panels(),
        };
        self.next_workspace_event_serial += 1;
        self.workspace_events.push(event);
    }

    pub fn workspace_event_trace(&self) -> WorkspacePaneEventTrace {
        WorkspacePaneEventTrace {
            events: self.workspace_events.clone(),
        }
    }
}
