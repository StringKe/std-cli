use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
        (Locale::ZhCn, "launcher.action.filter.hint") => Some("Filter actions"),
        (Locale::EnUs, "launcher.action.filter.hint") => Some("Filter actions"),
        (Locale::ZhCn, "launcher.action.filter.a11y") => Some("Action Panel filter"),
        (Locale::EnUs, "launcher.action.filter.a11y") => Some("Action Panel filter"),
        (Locale::ZhCn, "launcher.action.no_matches") => Some("No matching actions"),
        (Locale::EnUs, "launcher.action.no_matches") => Some("No matching actions"),
        (Locale::ZhCn, "launcher.feedback.copy") => Some("复制"),
        (Locale::EnUs, "launcher.feedback.copy") => Some("Copy"),
        (Locale::ZhCn, "launcher.feedback.retry") => Some("重试"),
        (Locale::EnUs, "launcher.feedback.retry") => Some("Retry"),
        (Locale::ZhCn, "launcher.feedback.open_studio") => Some("打开 Studio"),
        (Locale::EnUs, "launcher.feedback.open_studio") => Some("Open Studio"),
        (Locale::ZhCn, "launcher.feedback.completed") => Some("已完成"),
        (Locale::EnUs, "launcher.feedback.completed") => Some("Completed"),
        (Locale::ZhCn, "launcher.feedback.deferred") => Some("需要确认"),
        (Locale::EnUs, "launcher.feedback.deferred") => Some("Needs review"),
        (Locale::ZhCn, "launcher.feedback.failed") => Some("无法执行"),
        (Locale::EnUs, "launcher.feedback.failed") => Some("Unable to run"),
        _ => None,
    }
}

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    match key {
        "launcher.action.filter.hint" => Some("Filter actions"),
        "launcher.action.filter.a11y" => Some("Action Panel filter"),
        "launcher.action.no_matches" => Some("No matching actions"),
        "launcher.feedback.copy" => Some("Copy"),
        "launcher.feedback.retry" => Some("Retry"),
        "launcher.feedback.open_studio" => Some("Open Studio"),
        "launcher.feedback.completed" => Some("Completed"),
        "launcher.feedback.deferred" => Some("Needs review"),
        "launcher.feedback.failed" => Some("Unable to run"),
        _ => None,
    }
}
