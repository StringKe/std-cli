use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
        (Locale::ZhCn, "launcher.empty.no_matches.title") => Some("没有匹配项"),
        (Locale::EnUs, "launcher.empty.no_matches.title") => Some("No matches"),
        (Locale::ZhCn, "launcher.empty.no_matches.detail") => Some("换个关键词，或按 ? 询问"),
        (Locale::EnUs, "launcher.empty.no_matches.detail") => {
            Some("Try a different keyword or press ? to ask")
        }
        (Locale::ZhCn, "launcher.empty.ready.title") => Some("准备搜索"),
        (Locale::EnUs, "launcher.empty.ready.title") => Some("Ready to search"),
        (Locale::ZhCn, "launcher.empty.ready.detail") => {
            Some("按 / 查看命令，按 ? 询问，按下方向键查看最近使用")
        }
        (Locale::EnUs, "launcher.empty.ready.detail") => {
            Some("Press / for commands, ? to ask, Down for recent")
        }
        (Locale::ZhCn, "launcher.empty.ask_ai") => Some("询问 AI 关于"),
        (Locale::EnUs, "launcher.empty.ask_ai") => Some("Ask AI about"),
        (Locale::ZhCn, "launcher.results.searching") => Some("正在搜索 registry 和本地 index"),
        (Locale::EnUs, "launcher.results.searching") => Some("Searching registry and local index"),
        (Locale::ZhCn, "launcher.results.searching.title") => Some("Searching"),
        (Locale::EnUs, "launcher.results.searching.title") => Some("Searching"),
        (Locale::ZhCn, "launcher.results.executing.title") => Some("Executing"),
        (Locale::EnUs, "launcher.results.executing.title") => Some("Executing"),
        (Locale::ZhCn, "launcher.results.feedback.title") => Some("Result"),
        (Locale::EnUs, "launcher.results.feedback.title") => Some("Result"),
        (Locale::ZhCn, "launcher.results.nl.title") => Some("Ask"),
        (Locale::EnUs, "launcher.results.nl.title") => Some("Ask"),
        (Locale::ZhCn, "launcher.results.suggested_workflows.title") => Some("Suggested Workflows"),
        (Locale::EnUs, "launcher.results.suggested_workflows.title") => Some("Suggested Workflows"),
        (Locale::ZhCn, "launcher.empty.suggestion.rebuild.title") => Some("重建 Index"),
        (Locale::EnUs, "launcher.empty.suggestion.rebuild.title") => Some("Rebuild Index"),
        (Locale::ZhCn, "launcher.empty.suggestion.rebuild.detail") => Some("刷新本地项目搜索数据"),
        (Locale::EnUs, "launcher.empty.suggestion.rebuild.detail") => {
            Some("Refresh local project search data")
        }
        (Locale::ZhCn, "launcher.empty.suggestion.ask.title") => Some("询问项目"),
        (Locale::EnUs, "launcher.empty.suggestion.ask.title") => Some("Ask Project"),
        (Locale::ZhCn, "launcher.empty.suggestion.ask.detail") => Some("开始自然语言分析查询"),
        (Locale::EnUs, "launcher.empty.suggestion.ask.detail") => {
            Some("Start a natural language analysis query")
        }
        (Locale::ZhCn, "launcher.empty.suggestion.studio.title") => Some("打开 Studio"),
        (Locale::EnUs, "launcher.empty.suggestion.studio.title") => Some("Open Studio"),
        (Locale::ZhCn, "launcher.empty.suggestion.studio.detail") => Some("进入完整工作台"),
        (Locale::EnUs, "launcher.empty.suggestion.studio.detail") => {
            Some("Continue in the full workspace")
        }
        _ => result_list_translate(locale, key),
    }
}

fn result_list_translate(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
        (Locale::ZhCn, "launcher.results.title") => Some("Results"),
        (Locale::EnUs, "launcher.results.title") => Some("Results"),
        (Locale::ZhCn, "launcher.results.matches_suffix") => Some("matches"),
        (Locale::EnUs, "launcher.results.matches_suffix") => Some("matches"),
        (Locale::ZhCn, "launcher.results.overflow_hint") => Some("200+ matches, refine your query"),
        (Locale::EnUs, "launcher.results.overflow_hint") => Some("200+ matches, refine your query"),
        (Locale::ZhCn, "launcher.results.group.app_file") => Some("App / File"),
        (Locale::EnUs, "launcher.results.group.app_file") => Some("App / File"),
        (Locale::ZhCn, "launcher.results.group.action_workflow") => Some("Action / Workflow"),
        (Locale::EnUs, "launcher.results.group.action_workflow") => Some("Action / Workflow"),
        (Locale::ZhCn, "launcher.results.group.memory") => Some("Memory"),
        (Locale::EnUs, "launcher.results.group.memory") => Some("Memory"),
        (Locale::ZhCn, "launcher.results.group.skill") => Some("Skill"),
        (Locale::EnUs, "launcher.results.group.skill") => Some("Skill"),
        (Locale::ZhCn, "launcher.results.group.clipboard") => Some("Clipboard"),
        (Locale::EnUs, "launcher.results.group.clipboard") => Some("Clipboard"),
        (Locale::ZhCn, "launcher.results.group.other") => Some("Other"),
        (Locale::EnUs, "launcher.results.group.other") => Some("Other"),
        (Locale::ZhCn, "launcher.results.kind.app") => Some("App"),
        (Locale::EnUs, "launcher.results.kind.app") => Some("App"),
        (Locale::ZhCn, "launcher.results.kind.workflow") => Some("Workflow"),
        (Locale::EnUs, "launcher.results.kind.workflow") => Some("Workflow"),
        (Locale::ZhCn, "launcher.results.kind.command") => Some("Command"),
        (Locale::EnUs, "launcher.results.kind.command") => Some("Command"),
        (Locale::ZhCn, "launcher.results.kind.skill") => Some("Skill"),
        (Locale::EnUs, "launcher.results.kind.skill") => Some("Skill"),
        (Locale::ZhCn, "launcher.results.kind.memory") => Some("Memory"),
        (Locale::EnUs, "launcher.results.kind.memory") => Some("Memory"),
        (Locale::ZhCn, "launcher.results.kind.clipboard") => Some("Clipboard"),
        (Locale::EnUs, "launcher.results.kind.clipboard") => Some("Clipboard"),
        (Locale::ZhCn, "launcher.results.kind.file") => Some("File"),
        (Locale::EnUs, "launcher.results.kind.file") => Some("File"),
        (Locale::ZhCn, "launcher.results.kind.custom") => Some("Custom"),
        (Locale::EnUs, "launcher.results.kind.custom") => Some("Custom"),
        _ => None,
    }
}

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    match key {
        "launcher.empty.no_matches.title" => Some("No matches"),
        "launcher.empty.no_matches.detail" => Some("Try a different keyword or press ? to ask"),
        "launcher.empty.ready.title" => Some("Ready to search"),
        "launcher.empty.ready.detail" => Some("Press / for commands, ? to ask, Down for recent"),
        "launcher.empty.ask_ai" => Some("Ask AI about"),
        "launcher.results.searching" => Some("Searching registry and local index"),
        "launcher.results.searching.title" => Some("Searching"),
        "launcher.results.executing.title" => Some("Executing"),
        "launcher.results.feedback.title" => Some("Result"),
        "launcher.results.nl.title" => Some("Ask"),
        "launcher.results.suggested_workflows.title" => Some("Suggested Workflows"),
        "launcher.empty.suggestion.rebuild.title" => Some("Rebuild Index"),
        "launcher.empty.suggestion.rebuild.detail" => Some("Refresh local project search data"),
        "launcher.empty.suggestion.ask.title" => Some("Ask Project"),
        "launcher.empty.suggestion.ask.detail" => Some("Start a natural language analysis query"),
        "launcher.empty.suggestion.studio.title" => Some("Open Studio"),
        "launcher.empty.suggestion.studio.detail" => Some("Continue in the full workspace"),
        _ => result_list_fallback(key),
    }
}

fn result_list_fallback(key: &str) -> Option<&'static str> {
    match key {
        "launcher.results.title" => Some("Results"),
        "launcher.results.matches_suffix" => Some("matches"),
        "launcher.results.overflow_hint" => Some("200+ matches, refine your query"),
        "launcher.results.group.app_file" => Some("App / File"),
        "launcher.results.group.action_workflow" => Some("Action / Workflow"),
        "launcher.results.group.memory" => Some("Memory"),
        "launcher.results.group.skill" => Some("Skill"),
        "launcher.results.group.clipboard" => Some("Clipboard"),
        "launcher.results.group.other" => Some("Other"),
        "launcher.results.kind.app" => Some("App"),
        "launcher.results.kind.workflow" => Some("Workflow"),
        "launcher.results.kind.command" => Some("Command"),
        "launcher.results.kind.skill" => Some("Skill"),
        "launcher.results.kind.memory" => Some("Memory"),
        "launcher.results.kind.clipboard" => Some("Clipboard"),
        "launcher.results.kind.file" => Some("File"),
        "launcher.results.kind.custom" => Some("Custom"),
        _ => None,
    }
}
