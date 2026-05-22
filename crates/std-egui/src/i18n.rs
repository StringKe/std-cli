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
    use std::collections::BTreeSet;

    const CATALOG_SOURCES: [&str; 15] = [
        include_str!("i18n/catalog/launcher/feedback.rs"),
        include_str!("i18n/catalog/launcher/results.rs"),
        include_str!("i18n/catalog/launcher/search.rs"),
        include_str!("i18n/catalog/studio/analysis.rs"),
        include_str!("i18n/catalog/studio/apps.rs"),
        include_str!("i18n/catalog/studio/dashboard.rs"),
        include_str!("i18n/catalog/studio/history.rs"),
        include_str!("i18n/catalog/studio/memory.rs"),
        include_str!("i18n/catalog/studio/operations.rs"),
        include_str!("i18n/catalog/studio/plugins.rs"),
        include_str!("i18n/catalog/studio/settings.rs"),
        include_str!("i18n/catalog/studio/shell.rs"),
        include_str!("i18n/catalog/studio/workflow_builder.rs"),
        include_str!("i18n/catalog/studio/workflows.rs"),
        include_str!("i18n/catalog/studio/workspace_panes.rs"),
    ];

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
        assert_eq!(translate(Locale::EnUs, "launcher.search.icon"), "Search");
        assert_eq!(
            translate(Locale::EnUs, "launcher.search.loading"),
            "Searching"
        );
        assert_eq!(
            translate(Locale::ZhCn, "launcher.search.loading"),
            "正在搜索"
        );
        assert_eq!(
            translate(Locale::EnUs, "launcher.search.ime_composing"),
            "IME composing"
        );
        assert_eq!(
            translate(Locale::ZhCn, "launcher.search.ime_composing"),
            "输入法组合中"
        );
        assert_eq!(
            translate(Locale::EnUs, "launcher.feedback.failed"),
            "Unable to run"
        );
        assert_eq!(
            translate(Locale::ZhCn, "launcher.feedback.deferred"),
            "需要确认"
        );
        assert_eq!(
            translate(Locale::EnUs, "launcher.results.group.action_workflow"),
            "Action / Workflow"
        );
        assert_eq!(
            translate(Locale::ZhCn, "launcher.results.group.action_workflow"),
            "操作 / Workflow"
        );
        assert_eq!(
            translate(Locale::ZhCn, "launcher.results.searching.title"),
            "搜索中"
        );
        assert_eq!(
            translate(Locale::ZhCn, "launcher.results.matches_suffix"),
            "个匹配"
        );
        assert_eq!(
            translate(Locale::ZhCn, "launcher.results.kind.clipboard"),
            "剪贴板"
        );
        assert_eq!(
            translate(Locale::ZhCn, "launcher.empty.suggestion.rebuild.title"),
            "重建 Index"
        );
        assert_eq!(
            translate(Locale::EnUs, "launcher.empty.suggestion.studio.detail"),
            "Continue in the full workspace"
        );
        assert_eq!(
            translate(Locale::ZhCn, "launcher.action.filter.hint"),
            "过滤操作"
        );
        assert_eq!(
            translate(Locale::ZhCn, "launcher.action.filter.a11y"),
            "Action Panel 过滤"
        );
        assert_eq!(
            translate(Locale::ZhCn, "launcher.action.no_matches"),
            "没有匹配的操作"
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
        assert_eq!(
            translate(Locale::EnUs, "studio.settings.motion.reduce"),
            "Reduce motion"
        );
        assert_eq!(
            translate(Locale::ZhCn, "studio.settings.motion.reduce"),
            "减少动效"
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
        assert_eq!(
            translate(Locale::EnUs, "studio.apps.external_runner.status"),
            "NeedsExternalRunner"
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
    fn studio_workspace_panes_strings_have_zh_cn_and_en_us_values() {
        assert_eq!(
            translate(Locale::EnUs, "studio.workspace_panes.preview_workflow"),
            "Preview Workflow"
        );
        assert_eq!(
            translate(Locale::ZhCn, "studio.workspace_panes.detail"),
            "Studio 内部面板"
        );
        assert_eq!(
            translate(Locale::EnUs, "studio.workspace_panes.execution_history"),
            "Execution History"
        );
        assert_eq!(
            translate(Locale::EnUs, "studio.status.workspace_refreshed"),
            "Refreshed workspace state"
        );
        assert_eq!(
            translate(Locale::ZhCn, "studio.status.workspace_pane_opened"),
            "已打开 workspace pane"
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

    #[test]
    fn i18n_catalog_keys_have_zh_cn_en_us_and_fallback_values() {
        let keys = catalog_keys();
        assert!(keys.len() > 100, "catalog key audit is too narrow");

        for key in keys {
            assert!(
                catalog::translate(Locale::ZhCn, &key).is_some(),
                "missing zh-CN key: {key}"
            );
            assert!(
                catalog::translate(Locale::EnUs, &key).is_some(),
                "missing en-US key: {key}"
            );
            assert!(
                catalog::fallback(&key) != "UNKNOWN_I18N_KEY",
                "missing fallback key: {key}"
            );
        }
    }

    fn catalog_keys() -> BTreeSet<String> {
        CATALOG_SOURCES
            .iter()
            .flat_map(|source| source.split('"').skip(1).step_by(2))
            .filter(|value| value.starts_with("launcher.") || value.starts_with("studio."))
            .map(str::to_string)
            .collect()
    }
}
