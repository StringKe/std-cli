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
    pub const VIEWPORT_TOUCHPOINTS: &'static [&'static str] = &[
        "src/viewport.rs",
        "src/host_chrome.rs",
        "src/host_chrome_drag.rs",
        "src/preview.rs",
        "src/preview_tests.rs",
    ];
    pub const NATIVE_ENTRYPOINTS: &'static [&'static str] =
        &["src/native_app.rs", "src/preview.rs"];
    pub const FORBIDDEN_WORKBENCH_APIS: &'static [&'static str] = &[
        "egui::Window::new",
        "Window::new",
        "ViewportBuilder::default",
        "ViewportCommand::",
        "send_viewport_cmd",
        ".show_viewport",
        "ViewportId",
        "ViewportClass",
    ];

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

    pub fn strict_report(self) -> String {
        format!(
            "host={};pane_system={};native_child_windows={};detached_panels={};docs={};viewport_touchpoints={};native_entrypoints={};forbidden_apis={}",
            self.host_window.label(),
            self.pane_system.label(),
            self.native_child_windows,
            self.detached_panels,
            Self::DOC_REFERENCE,
            Self::VIEWPORT_TOUCHPOINTS.join("|"),
            Self::NATIVE_ENTRYPOINTS.join("|"),
            Self::FORBIDDEN_WORKBENCH_APIS.join("|")
        )
    }
}

impl HostWindowPolicy {
    pub const fn label(self) -> &'static str {
        match self {
            Self::SingleBorderlessEguiViewport => "single-borderless-egui-viewport",
        }
    }
}

impl PaneSystemPolicy {
    pub const fn label(self) -> &'static str {
        match self {
            Self::InternalEguiWorkspacePanes => "internal-egui-workspace-panes",
        }
    }
}
