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
    translate_launcher(locale, key)
        .or_else(|| translate_settings(locale, key))
        .or_else(|| translate_dashboard(locale, key))
        .or_else(|| translate_operations(locale, key))
        .unwrap_or_else(|| key_fallback(key))
}

fn translate_launcher(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
        (Locale::ZhCn, "launcher.search.placeholder") => Some("搜索 Workflow、应用、剪切板..."),
        (Locale::EnUs, "launcher.search.placeholder") => {
            Some("Search Workflows, apps, clipboard...")
        }
        (Locale::ZhCn, "launcher.empty.no_matches.title") => Some("没有匹配项"),
        (Locale::EnUs, "launcher.empty.no_matches.title") => Some("No matches"),
        (Locale::ZhCn, "launcher.empty.no_matches.detail") => Some("换个关键词，或按 ? 询问"),
        (Locale::EnUs, "launcher.empty.no_matches.detail") => {
            Some("Try a different keyword or press ? to ask")
        }
        (Locale::ZhCn, "launcher.empty.ready.title") => Some("准备搜索"),
        (Locale::EnUs, "launcher.empty.ready.title") => Some("Ready to search"),
        (Locale::ZhCn, "launcher.empty.ready.detail") => Some("输入关键词，或按 ? 询问"),
        (Locale::EnUs, "launcher.empty.ready.detail") => Some("Type a keyword, or press ? to ask"),
        (Locale::ZhCn, "launcher.empty.ask_ai") => Some("询问 AI 关于"),
        (Locale::EnUs, "launcher.empty.ask_ai") => Some("Ask AI about"),
        (Locale::ZhCn, "launcher.action.actions") => Some("操作"),
        (Locale::EnUs, "launcher.action.actions") => Some("Actions"),
        (Locale::ZhCn, "launcher.action.run") => Some("运行"),
        (Locale::EnUs, "launcher.action.run") => Some("Run"),
        _ => None,
    }
}

fn translate_settings(locale: Locale, key: &str) -> Option<&'static str> {
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

fn translate_dashboard(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
        (Locale::ZhCn, "studio.dashboard.title") => Some("Dashboard"),
        (Locale::EnUs, "studio.dashboard.title") => Some("Dashboard"),
        (Locale::ZhCn, "studio.dashboard.detail") => Some("本地自动化层的运行概览"),
        (Locale::EnUs, "studio.dashboard.detail") => {
            Some("Operational overview for the local automation layer")
        }
        (Locale::ZhCn, "studio.dashboard.actions") => Some("Action"),
        (Locale::EnUs, "studio.dashboard.actions") => Some("Actions"),
        (Locale::ZhCn, "studio.dashboard.actions.detail") => Some("可搜索单元"),
        (Locale::EnUs, "studio.dashboard.actions.detail") => Some("searchable units"),
        (Locale::ZhCn, "studio.dashboard.memory") => Some("Memory"),
        (Locale::EnUs, "studio.dashboard.memory") => Some("Memory"),
        (Locale::ZhCn, "studio.dashboard.memory.detail") => Some("本地记录"),
        (Locale::EnUs, "studio.dashboard.memory.detail") => Some("local notes"),
        (Locale::ZhCn, "studio.dashboard.audit_events") => Some("审计事件"),
        (Locale::EnUs, "studio.dashboard.audit_events") => Some("Audit Events"),
        (Locale::ZhCn, "studio.dashboard.audit_events.detail") => Some("事件轨迹"),
        (Locale::EnUs, "studio.dashboard.audit_events.detail") => Some("event trail"),
        (Locale::ZhCn, "studio.dashboard.planner.title") => Some("Planner 草稿"),
        (Locale::EnUs, "studio.dashboard.planner.title") => Some("Planner Draft"),
        (Locale::ZhCn, "studio.dashboard.planner.detail") => Some("AI 本地计划"),
        (Locale::EnUs, "studio.dashboard.planner.detail") => Some("AI local plan"),
        (Locale::ZhCn, "studio.dashboard.goal") => Some("目标"),
        (Locale::EnUs, "studio.dashboard.goal") => Some("Goal"),
        (Locale::ZhCn, "studio.dashboard.recent_memory.title") => Some("最近 Memory"),
        (Locale::EnUs, "studio.dashboard.recent_memory.title") => Some("Recent Memory"),
        (Locale::ZhCn, "studio.dashboard.recent_memory.detail") => Some("共享 core"),
        (Locale::EnUs, "studio.dashboard.recent_memory.detail") => Some("shared core"),
        (Locale::ZhCn, "studio.dashboard.recent_memory.empty") => Some("还没有 Memory 记录"),
        (Locale::EnUs, "studio.dashboard.recent_memory.empty") => Some("No memory records yet"),
        (Locale::ZhCn, "studio.dashboard.next_gates.title") => Some("下一批 gate"),
        (Locale::EnUs, "studio.dashboard.next_gates.title") => Some("Next Gates"),
        (Locale::ZhCn, "studio.dashboard.next_gates.detail") => Some("completion audit"),
        (Locale::EnUs, "studio.dashboard.next_gates.detail") => Some("completion audit"),
        (Locale::ZhCn, "studio.dashboard.gate.launcher") => Some("Launcher PASS 证据已存在"),
        (Locale::EnUs, "studio.dashboard.gate.launcher") => Some("Launcher PASS evidence exists"),
        (Locale::ZhCn, "studio.dashboard.gate.studio") => Some("Studio 真实 UI audit 进行中"),
        (Locale::EnUs, "studio.dashboard.gate.studio") => Some("Studio real UI audit ongoing"),
        (Locale::ZhCn, "studio.dashboard.gate.quality") => Some("质量工具只使用 Rust 生态"),
        (Locale::EnUs, "studio.dashboard.gate.quality") => Some("Quality rust ecosystem only"),
        _ => None,
    }
}

fn translate_operations(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
        (Locale::ZhCn, "studio.operations.title") => Some("Operations"),
        (Locale::EnUs, "studio.operations.title") => Some("Operations"),
        (Locale::ZhCn, "studio.operations.detail") => Some("QA、Doctor、Release、Install、Runtime"),
        (Locale::EnUs, "studio.operations.detail") => Some("QA, Doctor, Release, Install, Runtime"),
        (Locale::ZhCn, "studio.operations.command") => Some("命令"),
        (Locale::EnUs, "studio.operations.command") => Some("Command"),
        (Locale::ZhCn, "studio.operations.evidence") => Some("证据"),
        (Locale::EnUs, "studio.operations.evidence") => Some("Evidence"),
        (Locale::ZhCn, "studio.operations.current_workspace") => Some("当前 workspace 状态"),
        (Locale::EnUs, "studio.operations.current_workspace") => Some("current workspace state"),
        (Locale::ZhCn, "studio.operations.record_evidence") => Some("记录证据"),
        (Locale::EnUs, "studio.operations.record_evidence") => Some("Record Evidence"),
        (Locale::ZhCn, "studio.operations.completion.title") => Some("Completion Audit"),
        (Locale::EnUs, "studio.operations.completion.title") => Some("Completion Audit"),
        (Locale::ZhCn, "studio.operations.completion.detail") => Some("未证明前不算完成"),
        (Locale::EnUs, "studio.operations.completion.detail") => Some("not complete until proven"),
        (Locale::ZhCn, "studio.operations.completion.note") => {
            Some("每个区域都需要当前运行时证据才能完成")
        }
        (Locale::EnUs, "studio.operations.completion.note") => {
            Some("Each area requires current runtime evidence before completion.")
        }
        _ => None,
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
        "studio.dashboard.title" => "Dashboard",
        "studio.dashboard.detail" => "Operational overview for the local automation layer",
        "studio.dashboard.actions" => "Actions",
        "studio.dashboard.actions.detail" => "searchable units",
        "studio.dashboard.memory" => "Memory",
        "studio.dashboard.memory.detail" => "local notes",
        "studio.dashboard.audit_events" => "Audit Events",
        "studio.dashboard.audit_events.detail" => "event trail",
        "studio.dashboard.planner.title" => "Planner Draft",
        "studio.dashboard.planner.detail" => "AI local plan",
        "studio.dashboard.goal" => "Goal",
        "studio.dashboard.recent_memory.title" => "Recent Memory",
        "studio.dashboard.recent_memory.detail" => "shared core",
        "studio.dashboard.recent_memory.empty" => "No memory records yet",
        "studio.dashboard.next_gates.title" => "Next Gates",
        "studio.dashboard.next_gates.detail" => "completion audit",
        "studio.dashboard.gate.launcher" => "Launcher PASS evidence exists",
        "studio.dashboard.gate.studio" => "Studio real UI audit ongoing",
        "studio.dashboard.gate.quality" => "Quality rust ecosystem only",
        "studio.operations.title" => "Operations",
        "studio.operations.detail" => "QA, Doctor, Release, Install, Runtime",
        "studio.operations.command" => "Command",
        "studio.operations.evidence" => "Evidence",
        "studio.operations.current_workspace" => "current workspace state",
        "studio.operations.record_evidence" => "Record Evidence",
        "studio.operations.completion.title" => "Completion Audit",
        "studio.operations.completion.detail" => "not complete until proven",
        "studio.operations.completion.note" => {
            "Each area requires current runtime evidence before completion."
        }
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
}
