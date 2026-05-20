use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
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
        (Locale::ZhCn, "launcher.results.searching") => Some("正在搜索 registry 和本地 index"),
        (Locale::EnUs, "launcher.results.searching") => Some("Searching registry and local index"),
        _ => None,
    }
}

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    match key {
        "launcher.search.placeholder" => Some("Search Workflows, apps, clipboard..."),
        "launcher.empty.no_matches.title" => Some("No matches"),
        "launcher.empty.no_matches.detail" => Some("Try a different keyword or press ? to ask"),
        "launcher.empty.ready.title" => Some("Ready to search"),
        "launcher.empty.ready.detail" => Some("Type a keyword, or press ? to ask"),
        "launcher.empty.ask_ai" => Some("Ask AI about"),
        "launcher.action.actions" => Some("Actions"),
        "launcher.action.run" => Some("Run"),
        "launcher.action.executing" => Some("Executing selected action"),
        "launcher.action.command_hint" => Some("Press / for commands"),
        "launcher.voice.label" => Some("Voice"),
        "launcher.voice.placeholder" => Some("voice transcript"),
        "launcher.voice.apply" => Some("Apply"),
        "launcher.results.searching" => Some("Searching registry and local index"),
        _ => None,
    }
}
