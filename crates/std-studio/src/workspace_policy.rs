#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StudioWorkspacePolicy {
    pub host_window: HostWindowPolicy,
    pub pane_system: PaneSystemPolicy,
    pub native_child_windows: bool,
    pub detached_panels: bool,
    pub extra_viewports: bool,
    pub show_viewport_api: bool,
    pub egui_window_api: bool,
    pub settings_overlay: bool,
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
        "src/host_window.rs",
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
    pub const UI_COMPLETION_BOUNDARY: &'static str = "headless-smoke-is-not-ui-completion";
    pub const SOURCE_GUARD_CONTRACT: &'static str = "source_guard=full-src-scan;allowed_viewport_api=host-boundary-only;forbidden_workbench_api=egui-window|show-viewport|extra-viewport;settings=workspace-pane;open=internal-pane-intent";
    pub const MANUAL_UI_EVIDENCE_GATES: &'static [&'static str] = &[
        "light-dark-screenshots",
        "workspace-pane-open-focus-close-restore",
        "keyboard-a11y-focus",
        "operations-runtime-evidence",
    ];

    pub const fn studio_v1() -> Self {
        Self {
            host_window: HostWindowPolicy::SingleBorderlessEguiViewport,
            pane_system: PaneSystemPolicy::InternalEguiWorkspacePanes,
            native_child_windows: false,
            detached_panels: false,
            extra_viewports: false,
            show_viewport_api: false,
            egui_window_api: false,
            settings_overlay: false,
        }
    }

    pub const fn allows_native_child_windows(self) -> bool {
        self.native_child_windows
    }

    pub const fn allows_detached_panels(self) -> bool {
        self.detached_panels
    }

    pub const fn allows_extra_viewports(self) -> bool {
        self.extra_viewports
    }

    pub const fn allows_show_viewport_api(self) -> bool {
        self.show_viewport_api
    }

    pub const fn allows_egui_window_api(self) -> bool {
        self.egui_window_api
    }

    pub const fn allows_settings_overlay(self) -> bool {
        self.settings_overlay
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
            "host={};pane_system={};native_child_windows={};detached_panels={};extra_viewports={};show_viewport_api={};egui_window_api={};settings_overlay={};docs={};viewport_touchpoints={};native_entrypoints={};forbidden_apis={};ui_completion_boundary={};source_guard_contract={};manual_ui_evidence_gates={}",
            self.host_window.label(),
            self.pane_system.label(),
            self.native_child_windows,
            self.detached_panels,
            self.extra_viewports,
            self.show_viewport_api,
            self.egui_window_api,
            self.settings_overlay,
            Self::DOC_REFERENCE,
            Self::VIEWPORT_TOUCHPOINTS.join("|"),
            Self::NATIVE_ENTRYPOINTS.join("|"),
            Self::FORBIDDEN_WORKBENCH_APIS.join("|"),
            Self::UI_COMPLETION_BOUNDARY,
            Self::SOURCE_GUARD_CONTRACT,
            Self::MANUAL_UI_EVIDENCE_GATES.join("|")
        )
    }

    pub fn workspace_main_path_contract(self) -> String {
        format!(
            "host={};panes={};extra_viewports={};show_viewport=forbidden;show_viewport_api={};viewport_id=forbidden;egui_window=forbidden;egui_window_api={};settings_overlay=forbidden;settings_overlay={};allowed_viewport_files={}",
            self.host_window.label(),
            self.pane_system.label(),
            forbidden_label(self.allows_extra_viewports()),
            self.allows_show_viewport_api(),
            self.allows_egui_window_api(),
            self.allows_settings_overlay(),
            Self::VIEWPORT_TOUCHPOINTS.join("|")
        )
    }

    pub const fn host_window_command_boundary(self) -> &'static str {
        match self.host_window {
            HostWindowPolicy::SingleBorderlessEguiViewport => {
                "host_window_commands=single-system-host-only;workspace_panes=internal-egui-only;commands=close|minimize|maximize"
            }
        }
    }
}

const fn forbidden_label(allowed: bool) -> &'static str {
    if allowed {
        "allowed"
    } else {
        "forbidden"
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
