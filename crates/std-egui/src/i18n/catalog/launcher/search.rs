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
        (Locale::ZhCn, "launcher.search.loading") => Some("正在搜索"),
        (Locale::EnUs, "launcher.search.loading") => Some("Searching"),
        (Locale::ZhCn, "launcher.search.ime_composing") => Some("输入法组合中"),
        (Locale::EnUs, "launcher.search.ime_composing") => Some("IME composing"),
        (Locale::ZhCn, "launcher.action.actions") => Some("操作"),
        (Locale::EnUs, "launcher.action.actions") => Some("Actions"),
        (Locale::ZhCn, "launcher.action.run") => Some("运行"),
        (Locale::EnUs, "launcher.action.run") => Some("Run"),
        (Locale::ZhCn, "launcher.action.review_first") => Some("先检查"),
        (Locale::EnUs, "launcher.action.review_first") => Some("Review first"),
        (Locale::ZhCn, "launcher.action.defer") => Some("稍后执行"),
        (Locale::EnUs, "launcher.action.defer") => Some("Defer"),
        (Locale::ZhCn, "launcher.action.open_in_studio") => Some("在 Studio 打开"),
        (Locale::EnUs, "launcher.action.open_in_studio") => Some("Open in Studio"),
        (Locale::ZhCn, "launcher.action.copy_command") => Some("复制命令"),
        (Locale::EnUs, "launcher.action.copy_command") => Some("Copy command"),
        (Locale::ZhCn, "launcher.action.cancel") => Some("取消"),
        (Locale::EnUs, "launcher.action.cancel") => Some("Cancel"),
        (Locale::ZhCn, "launcher.action.background") => Some("移到后台"),
        (Locale::EnUs, "launcher.action.background") => Some("Move to background"),
        (Locale::ZhCn, "launcher.action.control.a11y") => Some("{label}，快捷键 {shortcut}"),
        (Locale::EnUs, "launcher.action.control.a11y") => Some("{label}, shortcut {shortcut}"),
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
        (Locale::ZhCn, "launcher.voice.empty_value") => Some("空"),
        (Locale::EnUs, "launcher.voice.empty_value") => Some("empty"),
        (Locale::ZhCn, "launcher.voice.input.a11y") => Some("{label}，文本框，当前值 {value}"),
        (Locale::EnUs, "launcher.voice.input.a11y") => Some("{label}, text box, value {value}"),
        (Locale::ZhCn, "launcher.a11y.search.empty") => Some("Launcher，搜索框，{placeholder}"),
        (Locale::EnUs, "launcher.a11y.search.empty") => {
            Some("Launcher, search field, {placeholder}")
        }
        (Locale::ZhCn, "launcher.a11y.search.query") => Some("Launcher，搜索框，{query}"),
        (Locale::EnUs, "launcher.a11y.search.query") => Some("Launcher, search field, {query}"),
        (Locale::ZhCn, "launcher.a11y.running") => Some("正在运行 {action}"),
        (Locale::EnUs, "launcher.a11y.running") => Some("Running {action}"),
        _ => None,
    }
}

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    match key {
        "launcher.search.placeholder" => Some("Search Workflows, apps, clipboard..."),
        "launcher.search.running" => Some("Running:"),
        "launcher.search.icon" => Some("Search"),
        "launcher.search.loading" => Some("Searching"),
        "launcher.search.ime_composing" => Some("IME composing"),
        "launcher.action.actions" => Some("Actions"),
        "launcher.action.run" => Some("Run"),
        "launcher.action.review_first" => Some("Review first"),
        "launcher.action.defer" => Some("Defer"),
        "launcher.action.open_in_studio" => Some("Open in Studio"),
        "launcher.action.copy_command" => Some("Copy command"),
        "launcher.action.cancel" => Some("Cancel"),
        "launcher.action.background" => Some("Move to background"),
        "launcher.action.control.a11y" => Some("{label}, shortcut {shortcut}"),
        "launcher.action.executing" => Some("Executing selected action"),
        "launcher.action.command_hint" => Some("Press / for commands"),
        "launcher.voice.label" => Some("Voice"),
        "launcher.voice.placeholder" => Some("voice transcript"),
        "launcher.voice.apply" => Some("Apply"),
        "launcher.voice.empty_value" => Some("empty"),
        "launcher.voice.input.a11y" => Some("{label}, text box, value {value}"),
        "launcher.a11y.search.empty" => Some("Launcher, search field, {placeholder}"),
        "launcher.a11y.search.query" => Some("Launcher, search field, {query}"),
        "launcher.a11y.running" => Some("Running {action}"),
        _ => None,
    }
}
