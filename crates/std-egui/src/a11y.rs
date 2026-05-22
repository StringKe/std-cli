use crate::i18n;

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
            return template(
                "launcher.a11y.search.empty",
                &[("{placeholder}", i18n::t("launcher.search.placeholder"))],
            );
        }
        template("launcher.a11y.search.query", &[("{query}", query.trim())])
    }

    pub fn launcher_result_label(
        &self,
        title: &str,
        subtitle: &str,
        position: usize,
        total: usize,
    ) -> String {
        template(
            "launcher.a11y.result",
            &[
                ("{title}", title),
                ("{subtitle}", subtitle),
                ("{position}", &position.to_string()),
                ("{total}", &total.to_string()),
            ],
        )
    }

    pub fn launcher_result_group_label(&self, group: &str) -> String {
        template("launcher.a11y.result_group", &[("{group}", group)])
    }

    pub fn launcher_action_panel_label(&self, selected_item: &str, count: usize) -> String {
        template(
            "launcher.a11y.action_panel",
            &[
                ("{selected}", selected_item),
                ("{count}", &count.to_string()),
            ],
        )
    }

    pub fn launcher_running_label(&self, action: &str) -> String {
        template("launcher.a11y.running", &[("{action}", action)])
    }

    pub fn launcher_completed_label(&self, message: &str) -> String {
        message.trim().to_string()
    }
}

fn template(key: &str, replacements: &[(&str, &str)]) -> String {
    let mut text = i18n::t(key).to_string();
    for (from, to) in replacements {
        text = text.replace(from, to);
    }
    text
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
            "Rebuild Index，Workflow，1 / 5，按 Enter 运行"
        );
        assert_eq!(
            a11y.launcher_result_group_label("操作 / Workflow"),
            "操作 / Workflow，结果分组"
        );
        assert_eq!(
            a11y.launcher_action_panel_label("Rebuild Index", 3),
            "Rebuild Index 的操作，列表 3 项"
        );
        assert_eq!(
            a11y.launcher_running_label("Rebuild Index"),
            "正在运行 Rebuild Index"
        );
        assert_eq!(
            a11y.launcher_completed_label("Index rebuilt"),
            "Index rebuilt"
        );
    }

    #[test]
    fn launcher_a11y_strings_have_en_us_fallbacks() {
        assert_eq!(
            i18n::translate(i18n::Locale::EnUs, "launcher.a11y.result"),
            "{title}, {subtitle}, {position} of {total}, press Enter to run"
        );
        assert_eq!(
            i18n::translate(i18n::Locale::EnUs, "launcher.a11y.action_panel"),
            "Actions for {selected}, list of {count}"
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
