use std::time::Duration;

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
}
