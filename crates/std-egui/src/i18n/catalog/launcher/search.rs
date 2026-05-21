use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
        (Locale::ZhCn, "launcher.search.placeholder") => Some("搜索 Workflow、应用、剪切板..."),
        (Locale::EnUs, "launcher.search.placeholder") => {
            Some("Search Workflows, apps, clipboard...")
        }
        (Locale::ZhCn, "launcher.search.running") => Some("正在执行："),
        (Locale::EnUs, "launcher.search.running") => Some("Running:"),
        (Locale::ZhCn, "launcher.search.icon") => Some("搜索"),
        (Locale::EnUs, "launcher.search.icon") => Some("Search"),
        (Locale::ZhCn, "launcher.action.actions") => Some("操作"),
        (Locale::EnUs, "launcher.action.actions") => Some("Actions"),
        (Locale::ZhCn, "launcher.action.run") => Some("运行"),
        (Locale::EnUs, "launcher.action.run") => Some("Run"),
        (Locale::ZhCn, "launcher.action.cancel") => Some("取消"),
        (Locale::EnUs, "launcher.action.cancel") => Some("Cancel"),
        (Locale::ZhCn, "launcher.action.background") => Some("移到后台"),
        (Locale::EnUs, "launcher.action.background") => Some("Move to background"),
        (Locale::ZhCn, "launcher.action.executing") => Some("正在执行选中操作"),
        (Locale::EnUs, "launcher.action.executing") => Some("Executing selected action"),
        (Locale::ZhCn, "launcher.action.command_hint") => Some("按 / 查看命令"),
        (Locale::EnUs, "launcher.action.command_hint") => Some("Press / for commands"),
        (Locale::ZhCn, "launcher.voice.label") => Some("语音"),
        (Locale::EnUs, "launcher.voice.label") => Some("Voice"),
        (Locale::ZhCn, "launcher.voice.placeholder") => Some("语音转写"),
        (Locale::EnUs, "launcher.voice.placeholder") => Some("voice transcript"),
        (Locale::ZhCn, "launcher.voice.apply") => Some("应用"),
        (Locale::EnUs, "launcher.voice.apply") => Some("Apply"),
        _ => None,
    }
}

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    match key {
        "launcher.search.placeholder" => Some("Search Workflows, apps, clipboard..."),
        "launcher.search.running" => Some("Running:"),
        "launcher.search.icon" => Some("Search"),
        "launcher.action.actions" => Some("Actions"),
        "launcher.action.run" => Some("Run"),
        "launcher.action.cancel" => Some("Cancel"),
        "launcher.action.background" => Some("Move to background"),
        "launcher.action.executing" => Some("Executing selected action"),
        "launcher.action.command_hint" => Some("Press / for commands"),
        "launcher.voice.label" => Some("Voice"),
        "launcher.voice.placeholder" => Some("voice transcript"),
        "launcher.voice.apply" => Some("Apply"),
        _ => None,
    }
}
