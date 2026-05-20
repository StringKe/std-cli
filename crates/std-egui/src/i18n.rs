mod catalog;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locale {
    ZhCn,
    EnUs,
}

impl Locale {
    pub fn from_env() -> Self {
        match std::env::var("STD_LOCALE").unwrap_or_default().as_str() {
            "en-US" | "en_US" | "en" => Self::EnUs,
            _ => Self::ZhCn,
        }
    }
}

pub fn t(key: &str) -> &'static str {
    translate(Locale::from_env(), key)
}

pub fn translate(locale: Locale, key: &str) -> &'static str {
    catalog::translate(locale, key).unwrap_or_else(|| catalog::fallback(key))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launcher_strings_have_zh_cn_and_en_us_values() {
        assert_eq!(
            translate(Locale::EnUs, "launcher.empty.no_matches.title"),
            "No matches"
        );
        assert_eq!(
            translate(Locale::ZhCn, "launcher.empty.no_matches.title"),
            "没有匹配项"
        );
        assert_eq!(
            translate(Locale::EnUs, "launcher.action.executing"),
            "Executing selected action"
        );
        assert_eq!(
            translate(Locale::ZhCn, "launcher.action.executing"),
            "正在执行选中操作"
        );
        assert_eq!(
            translate(Locale::EnUs, "launcher.results.group.action_workflow"),
            "Action / Workflow"
        );
        assert_eq!(
            translate(Locale::ZhCn, "launcher.action.filter.hint"),
            "Filter actions"
        );
    }

    #[test]
    fn unknown_keys_are_visible() {
        assert_eq!(translate(Locale::EnUs, "missing.key"), "UNKNOWN_I18N_KEY");
    }

    #[test]
    fn studio_settings_strings_have_zh_cn_and_en_us_values() {
        assert_eq!(translate(Locale::EnUs, "studio.settings.title"), "Settings");
        assert_eq!(translate(Locale::ZhCn, "studio.settings.title"), "设置");
        assert_eq!(
            translate(Locale::EnUs, "studio.settings.storage.note"),
            "StdConfig writes and reloads shared core state"
        );
    }

    #[test]
    fn studio_dashboard_strings_have_zh_cn_and_en_us_values() {
        assert_eq!(
            translate(Locale::EnUs, "studio.dashboard.gate.quality"),
            "Quality rust ecosystem only"
        );
        assert_eq!(
            translate(Locale::ZhCn, "studio.dashboard.gate.quality"),
            "质量工具只使用 Rust 生态"
        );
    }

    #[test]
    fn studio_operations_strings_have_zh_cn_and_en_us_values() {
        assert_eq!(
            translate(Locale::EnUs, "studio.operations.completion.note"),
            "Each area requires current runtime evidence before completion."
        );
        assert_eq!(
            translate(Locale::ZhCn, "studio.operations.completion.note"),
            "每个区域都需要当前运行时证据才能完成"
        );
    }

    #[test]
    fn studio_workflows_strings_have_zh_cn_and_en_us_values() {
        assert_eq!(
            translate(Locale::EnUs, "studio.workflows.run_batch"),
            "Run Batch"
        );
        assert_eq!(
            translate(Locale::ZhCn, "studio.workflows.run_batch"),
            "运行 Batch"
        );
    }

    #[test]
    fn studio_plugins_strings_have_zh_cn_and_en_us_values() {
        assert_eq!(
            translate(Locale::EnUs, "studio.plugins.execution.empty"),
            "No execution yet"
        );
        assert_eq!(
            translate(Locale::ZhCn, "studio.plugins.execution.empty"),
            "还没有执行记录"
        );
    }

    #[test]
    fn studio_apps_strings_have_zh_cn_and_en_us_values() {
        assert_eq!(
            translate(Locale::EnUs, "studio.apps.external_defer"),
            "external launch defaults to NeedsExternalRunner"
        );
        assert_eq!(
            translate(Locale::ZhCn, "studio.apps.external_defer"),
            "外部启动默认返回 NeedsExternalRunner"
        );
    }

    #[test]
    fn studio_history_strings_have_zh_cn_and_en_us_values() {
        assert_eq!(
            translate(Locale::EnUs, "studio.history.events.empty"),
            "No audit events"
        );
        assert_eq!(
            translate(Locale::ZhCn, "studio.history.events.empty"),
            "没有 audit event"
        );
    }

    #[test]
    fn studio_memory_strings_have_zh_cn_and_en_us_values() {
        assert_eq!(
            translate(Locale::EnUs, "studio.memory.detail.empty"),
            "Select a memory"
        );
        assert_eq!(
            translate(Locale::ZhCn, "studio.memory.detail.empty"),
            "选择一条 memory"
        );
    }

    #[test]
    fn studio_workflow_builder_strings_have_zh_cn_and_en_us_values() {
        assert_eq!(
            translate(Locale::EnUs, "studio.workflow_builder.properties.empty"),
            "Select a saved workflow to edit steps"
        );
        assert_eq!(
            translate(Locale::ZhCn, "studio.workflow_builder.properties.empty"),
            "选择已保存 workflow 以编辑步骤"
        );
    }

    #[test]
    fn studio_analysis_strings_have_zh_cn_and_en_us_values() {
        assert_eq!(
            translate(Locale::EnUs, "studio.analysis.coverage.report"),
            "Coverage Report"
        );
        assert_eq!(
            translate(Locale::ZhCn, "studio.analysis.output.empty"),
            "没有输出"
        );
    }

    #[test]
    fn studio_windows_strings_have_zh_cn_and_en_us_values() {
        assert_eq!(
            translate(Locale::EnUs, "studio.windows.preview_workflow"),
            "Preview Workflow"
        );
        assert_eq!(
            translate(Locale::ZhCn, "studio.windows.detail"),
            "Studio 内部面板"
        );
    }

    #[test]
    fn studio_shell_strings_have_zh_cn_and_en_us_values() {
        assert_eq!(
            translate(Locale::EnUs, "studio.shell.workspace.detail"),
            "main views"
        );
        assert_eq!(
            translate(Locale::ZhCn, "studio.shell.workspace.detail"),
            "主视图"
        );
        assert_eq!(
            translate(Locale::EnUs, "studio.shell.close_pane"),
            "Close Pane"
        );
        assert_eq!(
            translate(Locale::ZhCn, "studio.shell.quick_open.title"),
            "Quick Open"
        );
        assert_eq!(
            translate(Locale::EnUs, "studio.shell.command.open_settings"),
            "Open Settings"
        );
        assert_eq!(
            translate(Locale::ZhCn, "studio.shell.pane_inactive"),
            "inactive"
        );
    }
}
