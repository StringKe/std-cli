use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
        (Locale::ZhCn, "studio.settings.title") => Some("设置"),
        (Locale::EnUs, "studio.settings.title") => Some("Settings"),
        (Locale::ZhCn, "studio.settings.detail") => Some("共享配置与解析路径"),
        (Locale::EnUs, "studio.settings.detail") => Some("Shared configuration and resolved paths"),
        (Locale::ZhCn, "studio.settings.runtime.title") => Some("运行时"),
        (Locale::EnUs, "studio.settings.runtime.title") => Some("Runtime"),
        (Locale::ZhCn, "studio.settings.runtime.detail") => Some("Launcher 与 AI"),
        (Locale::EnUs, "studio.settings.runtime.detail") => Some("Launcher and AI"),
        (Locale::ZhCn, "studio.settings.hotkey.label") => Some("Launcher 快捷键"),
        (Locale::EnUs, "studio.settings.hotkey.label") => Some("Launcher hotkey"),
        (Locale::ZhCn, "studio.settings.hotkey.save") => Some("保存快捷键"),
        (Locale::EnUs, "studio.settings.hotkey.save") => Some("Save Hotkey"),
        (Locale::ZhCn, "studio.settings.ai.enable") => Some("启用 AI planner"),
        (Locale::EnUs, "studio.settings.ai.enable") => Some("Enable AI planner"),
        (Locale::ZhCn, "studio.settings.ai.save") => Some("保存 AI"),
        (Locale::EnUs, "studio.settings.ai.save") => Some("Save AI"),
        (Locale::ZhCn, "studio.settings.theme.label") => Some("主题"),
        (Locale::EnUs, "studio.settings.theme.label") => Some("Theme"),
        (Locale::ZhCn, "studio.settings.theme.save") => Some("保存主题"),
        (Locale::EnUs, "studio.settings.theme.save") => Some("Save Theme"),
        (Locale::ZhCn, "studio.settings.storage.title") => Some("存储"),
        (Locale::EnUs, "studio.settings.storage.title") => Some("Storage"),
        (Locale::ZhCn, "studio.settings.storage.detail") => Some("配置路径与数据根目录"),
        (Locale::EnUs, "studio.settings.storage.detail") => Some("Config path and data root"),
        (Locale::ZhCn, "studio.settings.data_dir.label") => Some("数据目录"),
        (Locale::EnUs, "studio.settings.data_dir.label") => Some("Data dir"),
        (Locale::ZhCn, "studio.settings.data_dir.save") => Some("保存数据目录"),
        (Locale::EnUs, "studio.settings.data_dir.save") => Some("Save Data Dir"),
        (Locale::ZhCn, "studio.settings.storage.note") => {
            Some("StdConfig 写入并重载共享 core 状态")
        }
        (Locale::EnUs, "studio.settings.storage.note") => {
            Some("StdConfig writes and reloads shared core state")
        }
        (Locale::ZhCn, "studio.settings.paths.title") => Some("解析路径"),
        (Locale::EnUs, "studio.settings.paths.title") => Some("Resolved Paths"),
        (Locale::ZhCn, "studio.settings.paths.detail") => Some("当前存储布局"),
        (Locale::EnUs, "studio.settings.paths.detail") => Some("Current storage layout"),
        (Locale::ZhCn, "studio.settings.saved") => Some("已保存"),
        (Locale::EnUs, "studio.settings.saved") => Some("saved"),
        _ => None,
    }
}

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    match key {
        "studio.settings.title" => Some("Settings"),
        "studio.settings.detail" => Some("Shared configuration and resolved paths"),
        "studio.settings.runtime.title" => Some("Runtime"),
        "studio.settings.runtime.detail" => Some("Launcher and AI"),
        "studio.settings.hotkey.label" => Some("Launcher hotkey"),
        "studio.settings.hotkey.save" => Some("Save Hotkey"),
        "studio.settings.ai.enable" => Some("Enable AI planner"),
        "studio.settings.ai.save" => Some("Save AI"),
        "studio.settings.theme.label" => Some("Theme"),
        "studio.settings.theme.save" => Some("Save Theme"),
        "studio.settings.storage.title" => Some("Storage"),
        "studio.settings.storage.detail" => Some("Config path and data root"),
        "studio.settings.data_dir.label" => Some("Data dir"),
        "studio.settings.data_dir.save" => Some("Save Data Dir"),
        "studio.settings.storage.note" => Some("StdConfig writes and reloads shared core state"),
        "studio.settings.paths.title" => Some("Resolved Paths"),
        "studio.settings.paths.detail" => Some("Current storage layout"),
        "studio.settings.saved" => Some("saved"),
        _ => None,
    }
}
