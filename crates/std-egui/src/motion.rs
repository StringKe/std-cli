use std::time::Duration;

pub const MOTION_FRAME_BUDGET_MS: u128 = 8;
pub const MOTION_ACTIVE_ANIMATION_LIMIT: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MotionCurve {
    Linear,
    OutStandard,
    InStandard,
    InOut,
    Snappy,
}

impl MotionCurve {
    pub fn token(self) -> &'static str {
        match self {
            Self::Linear => "ease/linear",
            Self::OutStandard => "ease/out-standard",
            Self::InStandard => "ease/in-standard",
            Self::InOut => "ease/in-out",
            Self::Snappy => "ease/snappy",
        }
    }

    pub fn cubic_bezier(self) -> Option<[f32; 4]> {
        match self {
            Self::Linear => None,
            Self::OutStandard => Some([0.2, 0.0, 0.0, 1.0]),
            Self::InStandard => Some([0.4, 0.0, 1.0, 1.0]),
            Self::InOut => Some([0.4, 0.0, 0.2, 1.0]),
            Self::Snappy => Some([0.18, 1.0, 0.22, 1.0]),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Durations {
    pub instant_ms: u64,
    pub micro_ms: u64,
    pub short_ms: u64,
    pub base_ms: u64,
    pub medium_ms: u64,
    pub long_ms: u64,
}

impl Durations {
    pub const STANDARD: Self = Self {
        instant_ms: 0,
        micro_ms: 80,
        short_ms: 140,
        base_ms: 220,
        medium_ms: 320,
        long_ms: 480,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionScene {
    Hover,
    Pressed,
    SelectedRow,
    FocusRing,
    TooltipEnter,
    TooltipExit,
    PopoverEnter,
    PopoverExit,
    SidebarToggle,
    LauncherEnter,
    LauncherExit,
    ModalEnter,
    ModalExit,
    ToastEnter,
    ToastExit,
    CollapsingHeader,
    TabSwitch,
    ListReplace,
    ProgressIndeterminate,
    ProgressDeterminate,
    DragSnap,
    ErrorShake,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MotionSpec {
    pub scene: MotionScene,
    pub duration: Duration,
    pub duration_token: &'static str,
    pub curve: MotionCurve,
    pub animated_properties: &'static str,
    pub reduced_behavior: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MotionBudgetReport {
    pub surface: &'static str,
    pub frame_p95_ms: u128,
    pub frame_budget_ms: u128,
    pub active_animations: usize,
    pub active_animation_limit: usize,
    pub sampled_frames: usize,
}

impl MotionBudgetReport {
    pub fn from_frame_samples(
        surface: &'static str,
        frame_times_ms: &[u128],
        active_animations: usize,
    ) -> Self {
        Self {
            surface,
            frame_p95_ms: percentile_95(frame_times_ms),
            frame_budget_ms: MOTION_FRAME_BUDGET_MS,
            active_animations,
            active_animation_limit: MOTION_ACTIVE_ANIMATION_LIMIT,
            sampled_frames: frame_times_ms.len(),
        }
    }

    pub fn pass(&self) -> bool {
        self.sampled_frames > 0
            && self.frame_p95_ms <= self.frame_budget_ms
            && self.active_animations <= self.active_animation_limit
    }

    pub fn summary(&self) -> String {
        format!(
            "{}_motion_budget {}\nframe_p95_ms={}\nframe_budget_ms={}\nactive_animations={}\nactive_animation_limit={}\nsampled_frames={}",
            self.surface,
            if self.pass() { "PASS" } else { "FAIL" },
            self.frame_p95_ms,
            self.frame_budget_ms,
            self.active_animations,
            self.active_animation_limit,
            self.sampled_frames
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MotionContext {
    reduced: bool,
    durations: Durations,
}

impl MotionContext {
    pub fn standard() -> Self {
        Self {
            reduced: false,
            durations: Durations::STANDARD,
        }
    }

    pub fn reduced() -> Self {
        Self {
            reduced: true,
            durations: Durations::STANDARD,
        }
    }

    pub fn from_env() -> Self {
        Self {
            reduced: reduce_motion_env(),
            durations: Durations::STANDARD,
        }
    }

    pub fn for_context(_ctx: &egui::Context) -> Self {
        Self::from_env()
    }

    pub fn is_reduced(self) -> bool {
        self.reduced
    }

    pub fn launcher_enter(self) -> Duration {
        self.duration_for_scene(MotionScene::LauncherEnter)
    }

    pub fn launcher_exit(self) -> Duration {
        self.duration_for_scene(MotionScene::LauncherExit)
    }

    pub fn focus_ring(self) -> Duration {
        self.duration_for_scene(MotionScene::FocusRing)
    }

    pub fn toast_enter(self) -> Duration {
        self.duration_for_scene(MotionScene::ToastEnter)
    }

    pub fn modal_enter(self) -> Duration {
        self.duration_for_scene(MotionScene::ModalEnter)
    }

    pub fn spec(self, scene: MotionScene) -> MotionSpec {
        let template = scene.template();
        MotionSpec {
            scene,
            duration: self.duration(template.duration_ms),
            duration_token: template.duration_token,
            curve: template.curve,
            animated_properties: template.animated_properties,
            reduced_behavior: template.reduced_behavior,
        }
    }

    pub fn scene_contract(self) -> String {
        MotionScene::ALL
            .iter()
            .map(|scene| {
                let spec = self.spec(*scene);
                format!(
                    "{}:{}:{}:{}",
                    scene.token(),
                    spec.duration.as_millis(),
                    spec.curve.token(),
                    spec.reduced_behavior
                )
            })
            .collect::<Vec<_>>()
            .join("|")
    }

    fn duration_for_scene(self, scene: MotionScene) -> Duration {
        self.spec(scene).duration
    }

    fn duration(self, ms: u64) -> Duration {
        if self.reduced {
            Duration::from_millis(self.durations.instant_ms)
        } else {
            Duration::from_millis(ms)
        }
    }
}

impl MotionScene {
    pub const ALL: [Self; 22] = [
        Self::Hover,
        Self::Pressed,
        Self::SelectedRow,
        Self::FocusRing,
        Self::TooltipEnter,
        Self::TooltipExit,
        Self::PopoverEnter,
        Self::PopoverExit,
        Self::SidebarToggle,
        Self::LauncherEnter,
        Self::LauncherExit,
        Self::ModalEnter,
        Self::ModalExit,
        Self::ToastEnter,
        Self::ToastExit,
        Self::CollapsingHeader,
        Self::TabSwitch,
        Self::ListReplace,
        Self::ProgressIndeterminate,
        Self::ProgressDeterminate,
        Self::DragSnap,
        Self::ErrorShake,
    ];

    pub fn token(self) -> &'static str {
        match self {
            Self::Hover => "hover",
            Self::Pressed => "pressed",
            Self::SelectedRow => "selected-row",
            Self::FocusRing => "focus-ring",
            Self::TooltipEnter => "tooltip-enter",
            Self::TooltipExit => "tooltip-exit",
            Self::PopoverEnter => "popover-enter",
            Self::PopoverExit => "popover-exit",
            Self::SidebarToggle => "sidebar-toggle",
            Self::LauncherEnter => "launcher-enter",
            Self::LauncherExit => "launcher-exit",
            Self::ModalEnter => "modal-enter",
            Self::ModalExit => "modal-exit",
            Self::ToastEnter => "toast-enter",
            Self::ToastExit => "toast-exit",
            Self::CollapsingHeader => "collapsing-header",
            Self::TabSwitch => "tab-switch",
            Self::ListReplace => "list-replace",
            Self::ProgressIndeterminate => "progress-indeterminate",
            Self::ProgressDeterminate => "progress-determinate",
            Self::DragSnap => "drag-snap",
            Self::ErrorShake => "error-shake",
        }
    }

    fn template(self) -> MotionSpecTemplate {
        use MotionCurve::{InOut, InStandard, Linear, OutStandard, Snappy};
        match self {
            Self::Hover => template(80, "dur/micro", OutStandard, "bg color", "color-instant"),
            Self::Pressed => template(80, "dur/micro", Snappy, "bg color + scale", "color-only"),
            Self::SelectedRow => template(140, "dur/short", OutStandard, "bg + text", "instant"),
            Self::FocusRing => template(140, "dur/short", OutStandard, "border alpha", "show"),
            Self::TooltipEnter => template(220, "dur/base", OutStandard, "opacity + y", "opacity"),
            Self::TooltipExit => template(80, "dur/micro", InStandard, "opacity", "instant"),
            Self::PopoverEnter => {
                template(220, "dur/base", OutStandard, "opacity + scale", "opacity")
            }
            Self::PopoverExit => {
                template(140, "dur/short", InStandard, "opacity + scale", "instant")
            }
            Self::SidebarToggle => template(220, "dur/base", InOut, "width", "instant"),
            Self::LauncherEnter => {
                template(320, "dur/medium", OutStandard, "opacity + y", "opacity")
            }
            Self::LauncherExit => {
                template(140, "dur/short", InStandard, "opacity + scale", "instant")
            }
            Self::ModalEnter => template(
                220,
                "dur/base",
                OutStandard,
                "overlay + scale + opacity",
                "direct",
            ),
            Self::ModalExit => template(
                140,
                "dur/short",
                InStandard,
                "overlay + scale + opacity",
                "instant",
            ),
            Self::ToastEnter => template(220, "dur/base", Snappy, "y + opacity", "opacity"),
            Self::ToastExit => template(80, "dur/micro", InStandard, "opacity", "instant"),
            Self::CollapsingHeader => template(220, "dur/base", InOut, "height + arrow", "instant"),
            Self::TabSwitch => template(140, "dur/short", OutStandard, "underline x", "instant"),
            Self::ListReplace => template(140, "dur/short", OutStandard, "opacity", "instant"),
            Self::ProgressIndeterminate => {
                template(1200, "dur/progress", Linear, "rotation", "static")
            }
            Self::ProgressDeterminate => {
                template(0, "dur/progress", Linear, "real percent", "preserve")
            }
            Self::DragSnap => template(140, "dur/short", OutStandard, "position", "instant"),
            Self::ErrorShake => template(240, "dur/custom", InOut, "x shake", "flash"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct MotionSpecTemplate {
    duration_ms: u64,
    duration_token: &'static str,
    curve: MotionCurve,
    animated_properties: &'static str,
    reduced_behavior: &'static str,
}

fn template(
    duration_ms: u64,
    duration_token: &'static str,
    curve: MotionCurve,
    animated_properties: &'static str,
    reduced_behavior: &'static str,
) -> MotionSpecTemplate {
    MotionSpecTemplate {
        duration_ms,
        duration_token,
        curve,
        animated_properties,
        reduced_behavior,
    }
}

pub fn reduce_motion_env() -> bool {
    std::env::var("STD_REDUCE_MOTION")
        .or_else(|_| std::env::var("STDCLI_REDUCE_MOTION"))
        .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "on"))
        .unwrap_or(false)
}

fn percentile_95(values: &[u128]) -> u128 {
    if values.is_empty() {
        return 0;
    }
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let index = ((sorted.len() - 1) * 95).div_ceil(100);
    sorted[index]
}
