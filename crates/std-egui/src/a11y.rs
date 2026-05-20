#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessibilityContext {
    pub reduce_motion: bool,
    pub reduce_transparency: bool,
    pub high_contrast: bool,
    pub bold_text: bool,
}

impl AccessibilityContext {
    pub fn from_env() -> Self {
        Self {
            reduce_motion: crate::motion::reduce_motion_env(),
            reduce_transparency: env_flag("STD_REDUCE_TRANSPARENCY"),
            high_contrast: env_flag("STD_HIGH_CONTRAST"),
            bold_text: env_flag("STD_BOLD_TEXT"),
        }
    }

    pub fn focus_ring_width(&self) -> f32 {
        if self.high_contrast {
            3.0
        } else {
            2.0
        }
    }

    pub fn launcher_search_label(&self, query: &str) -> String {
        if query.trim().is_empty() {
            "Launcher, search field, Search apps, workflows, memory, files".to_string()
        } else {
            format!("Launcher, search field, {}", query.trim())
        }
    }

    pub fn launcher_result_label(
        &self,
        title: &str,
        subtitle: &str,
        position: usize,
        total: usize,
    ) -> String {
        format!("{title}, {subtitle}, {position} of {total}, press Enter to run")
    }
}

fn env_flag(name: &str) -> bool {
    std::env::var(name)
        .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "on"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launcher_labels_include_required_screen_reader_context() {
        let a11y = AccessibilityContext {
            reduce_motion: false,
            reduce_transparency: false,
            high_contrast: false,
            bold_text: false,
        };

        assert!(a11y.launcher_search_label("").contains("Launcher"));
        assert_eq!(
            a11y.launcher_result_label("Rebuild Index", "Workflow", 1, 5),
            "Rebuild Index, Workflow, 1 of 5, press Enter to run"
        );
    }

    #[test]
    fn high_contrast_increases_focus_ring_width() {
        let a11y = AccessibilityContext {
            reduce_motion: false,
            reduce_transparency: false,
            high_contrast: true,
            bold_text: false,
        };

        assert_eq!(a11y.focus_ring_width(), 3.0);
    }
}
