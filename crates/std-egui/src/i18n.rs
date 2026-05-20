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
}
