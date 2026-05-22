use std::time::Duration;

pub const MOTION_FRAME_BUDGET_MS: u128 = 8;
pub const MOTION_ACTIVE_ANIMATION_LIMIT: usize = 8;

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
        self.duration(self.durations.medium_ms)
    }

    pub fn launcher_exit(self) -> Duration {
        self.duration(self.durations.short_ms)
    }

    pub fn focus_ring(self) -> Duration {
        self.duration(self.durations.short_ms)
    }

    pub fn toast_enter(self) -> Duration {
        self.duration(self.durations.base_ms)
    }

    pub fn modal_enter(self) -> Duration {
        self.duration(self.durations.base_ms)
    }

    fn duration(self, ms: u64) -> Duration {
        if self.reduced {
            Duration::from_millis(self.durations.instant_ms)
        } else {
            Duration::from_millis(ms)
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_motion_uses_documented_launcher_duration() {
        let motion = MotionContext::standard();

        assert_eq!(motion.launcher_enter(), Duration::from_millis(320));
        assert_eq!(motion.launcher_exit(), Duration::from_millis(140));
    }

    #[test]
    fn reduced_motion_collapses_nonessential_durations() {
        let motion = MotionContext::reduced();

        assert_eq!(motion.launcher_enter(), Duration::ZERO);
        assert_eq!(motion.focus_ring(), Duration::ZERO);
        assert_eq!(motion.modal_enter(), Duration::ZERO);
    }

    #[test]
    fn motion_budget_reports_p95_frame_time_and_animation_limit() {
        let report = MotionBudgetReport::from_frame_samples("launcher", &[2, 3, 4, 8, 12], 8);

        assert_eq!(report.frame_p95_ms, 12);
        assert!(!report.pass());
        assert!(report.summary().contains("launcher_motion_budget FAIL"));
    }

    #[test]
    fn motion_budget_passes_when_samples_stay_inside_docs19_limits() {
        let report = MotionBudgetReport::from_frame_samples("studio", &[2, 3, 4, 7, 8], 6);

        assert!(report.pass());
        assert!(report.summary().contains("studio_motion_budget PASS"));
        assert!(report.summary().contains("active_animation_limit=8"));
    }
}
