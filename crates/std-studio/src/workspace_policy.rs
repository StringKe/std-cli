#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StudioWorkspacePolicy {
    pub host_window: HostWindowPolicy,
    pub pane_system: PaneSystemPolicy,
    pub native_child_windows: bool,
    pub detached_panels: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostWindowPolicy {
    SingleBorderlessEguiViewport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneSystemPolicy {
    InternalEguiWorkspacePanes,
}

impl StudioWorkspacePolicy {
    pub const DOC_REFERENCE: &'static str = "docs/22 + docs/24";

    pub const fn studio_v1() -> Self {
        Self {
            host_window: HostWindowPolicy::SingleBorderlessEguiViewport,
            pane_system: PaneSystemPolicy::InternalEguiWorkspacePanes,
            native_child_windows: false,
            detached_panels: false,
        }
    }

    pub const fn allows_native_child_windows(self) -> bool {
        self.native_child_windows
    }

    pub const fn allows_detached_panels(self) -> bool {
        self.detached_panels
    }

    pub const fn summary(self) -> &'static str {
        match (self.host_window, self.pane_system) {
            (
                HostWindowPolicy::SingleBorderlessEguiViewport,
                PaneSystemPolicy::InternalEguiWorkspacePanes,
            ) => "single egui host viewport, internal workspace panes",
        }
    }
}
