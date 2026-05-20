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
    match (locale, key) {
        (Locale::ZhCn, "launcher.search.placeholder") => "搜索 Workflow、应用、剪切板...",
        (Locale::EnUs, "launcher.search.placeholder") => "Search Workflows, apps, clipboard...",
        (Locale::ZhCn, "launcher.empty.no_matches.title") => "没有匹配项",
        (Locale::EnUs, "launcher.empty.no_matches.title") => "No matches",
        (Locale::ZhCn, "launcher.empty.no_matches.detail") => "换个关键词，或按 ? 询问",
        (Locale::EnUs, "launcher.empty.no_matches.detail") => {
            "Try a different keyword or press ? to ask"
        }
        (Locale::ZhCn, "launcher.empty.ready.title") => "准备搜索",
        (Locale::EnUs, "launcher.empty.ready.title") => "Ready to search",
        (Locale::ZhCn, "launcher.empty.ready.detail") => "输入关键词，或按 ? 询问",
        (Locale::EnUs, "launcher.empty.ready.detail") => "Type a keyword, or press ? to ask",
        (Locale::ZhCn, "launcher.empty.ask_ai") => "询问 AI 关于",
        (Locale::EnUs, "launcher.empty.ask_ai") => "Ask AI about",
        (Locale::ZhCn, "launcher.action.actions") => "操作",
        (Locale::EnUs, "launcher.action.actions") => "Actions",
        (Locale::ZhCn, "launcher.action.run") => "运行",
        (Locale::EnUs, "launcher.action.run") => "Run",
        (Locale::ZhCn, "studio.settings.title") => "设置",
        (Locale::EnUs, "studio.settings.title") => "Settings",
        (Locale::ZhCn, "studio.settings.detail") => "共享配置与解析路径",
        (Locale::EnUs, "studio.settings.detail") => "Shared configuration and resolved paths",
        (Locale::ZhCn, "studio.settings.runtime.title") => "运行时",
        (Locale::EnUs, "studio.settings.runtime.title") => "Runtime",
        (Locale::ZhCn, "studio.settings.runtime.detail") => "Launcher 与 AI",
        (Locale::EnUs, "studio.settings.runtime.detail") => "Launcher and AI",
        (Locale::ZhCn, "studio.settings.hotkey.label") => "Launcher 快捷键",
        (Locale::EnUs, "studio.settings.hotkey.label") => "Launcher hotkey",
        (Locale::ZhCn, "studio.settings.hotkey.save") => "保存快捷键",
        (Locale::EnUs, "studio.settings.hotkey.save") => "Save Hotkey",
        (Locale::ZhCn, "studio.settings.ai.enable") => "启用 AI planner",
        (Locale::EnUs, "studio.settings.ai.enable") => "Enable AI planner",
        (Locale::ZhCn, "studio.settings.ai.save") => "保存 AI",
        (Locale::EnUs, "studio.settings.ai.save") => "Save AI",
        (Locale::ZhCn, "studio.settings.theme.label") => "主题",
        (Locale::EnUs, "studio.settings.theme.label") => "Theme",
        (Locale::ZhCn, "studio.settings.theme.save") => "保存主题",
        (Locale::EnUs, "studio.settings.theme.save") => "Save Theme",
        (Locale::ZhCn, "studio.settings.storage.title") => "存储",
        (Locale::EnUs, "studio.settings.storage.title") => "Storage",
        (Locale::ZhCn, "studio.settings.storage.detail") => "配置路径与数据根目录",
        (Locale::EnUs, "studio.settings.storage.detail") => "Config path and data root",
        (Locale::ZhCn, "studio.settings.data_dir.label") => "数据目录",
        (Locale::EnUs, "studio.settings.data_dir.label") => "Data dir",
        (Locale::ZhCn, "studio.settings.data_dir.save") => "保存数据目录",
        (Locale::EnUs, "studio.settings.data_dir.save") => "Save Data Dir",
        (Locale::ZhCn, "studio.settings.storage.note") => "StdConfig 写入并重载共享 core 状态",
        (Locale::EnUs, "studio.settings.storage.note") => {
            "StdConfig writes and reloads shared core state"
        }
        (Locale::ZhCn, "studio.settings.paths.title") => "解析路径",
        (Locale::EnUs, "studio.settings.paths.title") => "Resolved Paths",
        (Locale::ZhCn, "studio.settings.paths.detail") => "当前存储布局",
        (Locale::EnUs, "studio.settings.paths.detail") => "Current storage layout",
        (Locale::ZhCn, "studio.settings.saved") => "已保存",
        (Locale::EnUs, "studio.settings.saved") => "saved",
        _ => key_fallback(key),
    }
}

fn key_fallback(key: &str) -> &'static str {
    match key {
        "launcher.search.placeholder" => "Search Workflows, apps, clipboard...",
        "launcher.empty.no_matches.title" => "No matches",
        "launcher.empty.no_matches.detail" => "Try a different keyword or press ? to ask",
        "launcher.empty.ready.title" => "Ready to search",
        "launcher.empty.ready.detail" => "Type a keyword, or press ? to ask",
        "launcher.empty.ask_ai" => "Ask AI about",
        "launcher.action.actions" => "Actions",
        "launcher.action.run" => "Run",
        "studio.settings.title" => "Settings",
        "studio.settings.detail" => "Shared configuration and resolved paths",
        "studio.settings.runtime.title" => "Runtime",
        "studio.settings.runtime.detail" => "Launcher and AI",
        "studio.settings.hotkey.label" => "Launcher hotkey",
        "studio.settings.hotkey.save" => "Save Hotkey",
        "studio.settings.ai.enable" => "Enable AI planner",
        "studio.settings.ai.save" => "Save AI",
        "studio.settings.theme.label" => "Theme",
        "studio.settings.theme.save" => "Save Theme",
        "studio.settings.storage.title" => "Storage",
        "studio.settings.storage.detail" => "Config path and data root",
        "studio.settings.data_dir.label" => "Data dir",
        "studio.settings.data_dir.save" => "Save Data Dir",
        "studio.settings.storage.note" => "StdConfig writes and reloads shared core state",
        "studio.settings.paths.title" => "Resolved Paths",
        "studio.settings.paths.detail" => "Current storage layout",
        "studio.settings.saved" => "saved",
        _ => "UNKNOWN_I18N_KEY",
    }
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
}
