use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
    translate_primary(locale, key).or_else(|| translate_storage(locale, key))
}

fn translate_primary(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
        (Locale::ZhCn, "studio.settings.title") => Some("设置"),
        (Locale::EnUs, "studio.settings.title") => Some("Settings"),
        (Locale::ZhCn, "studio.settings.detail") => Some("共享配置与解析路径"),
        (Locale::EnUs, "studio.settings.detail") => Some("Shared configuration and resolved paths"),
        (Locale::ZhCn, "studio.settings.nav.title") => Some("分类"),
        (Locale::EnUs, "studio.settings.nav.title") => Some("Categories"),
        (Locale::ZhCn, "studio.settings.nav.detail") => Some("内部工作区"),
        (Locale::EnUs, "studio.settings.nav.detail") => Some("Workspace pane"),
        (Locale::ZhCn, "studio.settings.category.appearance") => Some("外观"),
        (Locale::EnUs, "studio.settings.category.appearance") => Some("Appearance"),
        (Locale::ZhCn, "studio.settings.category.appearance.detail") => Some("主题与视觉 token"),
        (Locale::EnUs, "studio.settings.category.appearance.detail") => {
            Some("Theme and visual tokens")
        }
        (Locale::ZhCn, "studio.settings.category.hotkeys") => Some("快捷键"),
        (Locale::EnUs, "studio.settings.category.hotkeys") => Some("Hotkeys"),
        (Locale::ZhCn, "studio.settings.category.hotkeys.detail") => {
            Some("Launcher 与 Studio 输入")
        }
        (Locale::EnUs, "studio.settings.category.hotkeys.detail") => {
            Some("Launcher and Studio input")
        }
        (Locale::ZhCn, "studio.settings.category.ai_provider") => Some("AI Provider"),
        (Locale::EnUs, "studio.settings.category.ai_provider") => Some("AI Provider"),
        (Locale::ZhCn, "studio.settings.category.ai_provider.detail") => Some("Planner 与辅助能力"),
        (Locale::EnUs, "studio.settings.category.ai_provider.detail") => Some("Planner and assist"),
        (Locale::ZhCn, "studio.settings.category.index") => Some("Index"),
        (Locale::EnUs, "studio.settings.category.index") => Some("Index"),
        (Locale::ZhCn, "studio.settings.category.index.detail") => Some("四层索引路径"),
        (Locale::EnUs, "studio.settings.category.index.detail") => Some("Four-layer index paths"),
        (Locale::ZhCn, "studio.settings.category.plugins") => Some("Plugins"),
        (Locale::EnUs, "studio.settings.category.plugins") => Some("Plugins"),
        (Locale::ZhCn, "studio.settings.category.plugins.detail") => Some("插件与本地存储"),
        (Locale::EnUs, "studio.settings.category.plugins.detail") => {
            Some("Plugins and local storage")
        }
        (Locale::ZhCn, "studio.settings.category.privacy") => Some("Privacy"),
        (Locale::EnUs, "studio.settings.category.privacy") => Some("Privacy"),
        (Locale::ZhCn, "studio.settings.category.privacy.detail") => Some("本地优先边界"),
        (Locale::EnUs, "studio.settings.category.privacy.detail") => Some("Local-first boundary"),
        (Locale::ZhCn, "studio.settings.category.about") => Some("About"),
        (Locale::EnUs, "studio.settings.category.about") => Some("About"),
        (Locale::ZhCn, "studio.settings.category.about.detail") => Some("产品与宿主策略"),
        (Locale::EnUs, "studio.settings.category.about.detail") => Some("Product and host policy"),
        (Locale::ZhCn, "studio.settings.runtime.title") => Some("运行时"),
        (Locale::EnUs, "studio.settings.runtime.title") => Some("Runtime"),
        (Locale::ZhCn, "studio.settings.runtime.detail") => Some("Launcher 与 AI"),
        (Locale::EnUs, "studio.settings.runtime.detail") => Some("Launcher and AI"),
        (Locale::ZhCn, "studio.settings.hotkey.label") => Some("Launcher 快捷键"),
        (Locale::EnUs, "studio.settings.hotkey.label") => Some("Launcher hotkey"),
        (Locale::ZhCn, "studio.settings.hotkey.save") => Some("保存快捷键"),
        (Locale::EnUs, "studio.settings.hotkey.save") => Some("Save Hotkey"),
        (Locale::ZhCn, "studio.settings.hotkey.registry.title") => Some("快捷键列表"),
        (Locale::EnUs, "studio.settings.hotkey.registry.title") => Some("Shortcut Registry"),
        (Locale::ZhCn, "studio.settings.hotkey.registry.detail") => Some("来源与默认值可见"),
        (Locale::EnUs, "studio.settings.hotkey.registry.detail") => {
            Some("Source and defaults visible")
        }
        (Locale::ZhCn, "studio.settings.hotkey.reset") => Some("重置"),
        (Locale::EnUs, "studio.settings.hotkey.reset") => Some("Reset"),
        (Locale::ZhCn, "studio.settings.hotkey.row") => Some("快捷键"),
        (Locale::EnUs, "studio.settings.hotkey.row") => Some("Shortcut"),
        (Locale::ZhCn, "studio.settings.ai.enable") => Some("启用 AI planner"),
        (Locale::EnUs, "studio.settings.ai.enable") => Some("Enable AI planner"),
        (Locale::ZhCn, "studio.settings.ai.detail") => Some("保存后同步 Planner 与 Studio 状态栏"),
        (Locale::EnUs, "studio.settings.ai.detail") => {
            Some("Saves planner state and Studio status bar")
        }
        (Locale::ZhCn, "studio.settings.ai.save") => Some("保存 AI"),
        (Locale::EnUs, "studio.settings.ai.save") => Some("Save AI"),
        (Locale::ZhCn, "studio.settings.toggle.on") => Some("开启"),
        (Locale::EnUs, "studio.settings.toggle.on") => Some("On"),
        (Locale::ZhCn, "studio.settings.toggle.off") => Some("关闭"),
        (Locale::EnUs, "studio.settings.toggle.off") => Some("Off"),
        (Locale::ZhCn, "studio.settings.theme.label") => Some("主题"),
        (Locale::EnUs, "studio.settings.theme.label") => Some("Theme"),
        (Locale::ZhCn, "studio.settings.theme.save") => Some("保存主题"),
        (Locale::EnUs, "studio.settings.theme.save") => Some("Save Theme"),
        (Locale::ZhCn, "studio.settings.theme.system") => Some("跟随系统"),
        (Locale::EnUs, "studio.settings.theme.system") => Some("System"),
        (Locale::ZhCn, "studio.settings.theme.dark") => Some("深色"),
        (Locale::EnUs, "studio.settings.theme.dark") => Some("Dark"),
        (Locale::ZhCn, "studio.settings.theme.light") => Some("浅色"),
        (Locale::EnUs, "studio.settings.theme.light") => Some("Light"),
        (Locale::ZhCn, "studio.settings.theme.active") => Some("当前主题"),
        (Locale::EnUs, "studio.settings.theme.active") => Some("Active theme"),
        (Locale::ZhCn, "studio.settings.theme.contract") => {
            Some("light / dark / system 使用同一套 token")
        }
        (Locale::EnUs, "studio.settings.theme.contract") => {
            Some("light / dark / system share one token set")
        }
        (Locale::ZhCn, "studio.settings.hotkey.contract") => {
            Some("焦点与 IME 优先，热键只显式注册")
        }
        (Locale::EnUs, "studio.settings.hotkey.contract") => {
            Some("Focus and IME first, hotkeys register by explicit opt-in")
        }
        _ => None,
    }
}

fn translate_storage(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
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
        (Locale::ZhCn, "studio.settings.privacy.contract") => {
            Some("默认 smoke 不启动 App、Terminal、截图或外部 runner")
        }
        (Locale::EnUs, "studio.settings.privacy.contract") => {
            Some("Default smoke never starts apps, Terminal, screenshots, or external runners")
        }
        (Locale::ZhCn, "studio.settings.about.product") => Some("std-cli Studio"),
        (Locale::EnUs, "studio.settings.about.product") => Some("std-cli Studio"),
        (Locale::ZhCn, "studio.settings.about.surface") => Some("单宿主 egui workspace pane"),
        (Locale::EnUs, "studio.settings.about.surface") => Some("Single-host egui workspace pane"),
        _ => None,
    }
}

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    match key {
        "studio.settings.title" => Some("Settings"),
        "studio.settings.detail" => Some("Shared configuration and resolved paths"),
        "studio.settings.nav.title" => Some("Categories"),
        "studio.settings.nav.detail" => Some("Workspace pane"),
        "studio.settings.category.appearance" => Some("Appearance"),
        "studio.settings.category.appearance.detail" => Some("Theme and visual tokens"),
        "studio.settings.category.hotkeys" => Some("Hotkeys"),
        "studio.settings.category.hotkeys.detail" => Some("Launcher and Studio input"),
        "studio.settings.category.ai_provider" => Some("AI Provider"),
        "studio.settings.category.ai_provider.detail" => Some("Planner and assist"),
        "studio.settings.category.index" => Some("Index"),
        "studio.settings.category.index.detail" => Some("Four-layer index paths"),
        "studio.settings.category.plugins" => Some("Plugins"),
        "studio.settings.category.plugins.detail" => Some("Plugins and local storage"),
        "studio.settings.category.privacy" => Some("Privacy"),
        "studio.settings.category.privacy.detail" => Some("Local-first boundary"),
        "studio.settings.category.about" => Some("About"),
        "studio.settings.category.about.detail" => Some("Product and host policy"),
        "studio.settings.runtime.title" => Some("Runtime"),
        "studio.settings.runtime.detail" => Some("Launcher and AI"),
        "studio.settings.hotkey.label" => Some("Launcher hotkey"),
        "studio.settings.hotkey.save" => Some("Save Hotkey"),
        "studio.settings.hotkey.registry.title" => Some("Shortcut Registry"),
        "studio.settings.hotkey.registry.detail" => Some("Source and defaults visible"),
        "studio.settings.hotkey.reset" => Some("Reset"),
        "studio.settings.hotkey.row" => Some("Shortcut"),
        "studio.settings.ai.enable" => Some("Enable AI planner"),
        "studio.settings.ai.detail" => Some("Saves planner state and Studio status bar"),
        "studio.settings.ai.save" => Some("Save AI"),
        "studio.settings.toggle.on" => Some("On"),
        "studio.settings.toggle.off" => Some("Off"),
        "studio.settings.theme.label" => Some("Theme"),
        "studio.settings.theme.save" => Some("Save Theme"),
        "studio.settings.theme.system" => Some("System"),
        "studio.settings.theme.dark" => Some("Dark"),
        "studio.settings.theme.light" => Some("Light"),
        "studio.settings.theme.active" => Some("Active theme"),
        "studio.settings.theme.contract" => Some("light / dark / system share one token set"),
        "studio.settings.hotkey.contract" => {
            Some("Focus and IME first, hotkeys register by explicit opt-in")
        }
        "studio.settings.storage.title" => Some("Storage"),
        "studio.settings.storage.detail" => Some("Config path and data root"),
        "studio.settings.data_dir.label" => Some("Data dir"),
        "studio.settings.data_dir.save" => Some("Save Data Dir"),
        "studio.settings.storage.note" => Some("StdConfig writes and reloads shared core state"),
        "studio.settings.paths.title" => Some("Resolved Paths"),
        "studio.settings.paths.detail" => Some("Current storage layout"),
        "studio.settings.saved" => Some("saved"),
        "studio.settings.privacy.contract" => {
            Some("Default smoke never starts apps, Terminal, screenshots, or external runners")
        }
        "studio.settings.about.product" => Some("std-cli Studio"),
        "studio.settings.about.surface" => Some("Single-host egui workspace pane"),
        _ => None,
    }
}
