use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
        (Locale::ZhCn, "launcher.action.filter.hint") => Some("过滤操作"),
        (Locale::EnUs, "launcher.action.filter.hint") => Some("Filter actions"),
        (Locale::ZhCn, "launcher.action.filter.a11y") => Some("Action Panel 过滤"),
        (Locale::EnUs, "launcher.action.filter.a11y") => Some("Action Panel filter"),
        (Locale::ZhCn, "launcher.action.filter.value.empty") => Some("空"),
        (Locale::EnUs, "launcher.action.filter.value.empty") => Some("empty"),
        (Locale::ZhCn, "launcher.action.filter.input.a11y") => {
            Some("{label}，文本框，当前值 {value}")
        }
        (Locale::EnUs, "launcher.action.filter.input.a11y") => {
            Some("{label}, text box, value {value}")
        }
        (Locale::ZhCn, "launcher.action.row.a11y") => Some("{label}，操作"),
        (Locale::EnUs, "launcher.action.row.a11y") => Some("{label} action"),
        (Locale::ZhCn, "launcher.action.no_matches") => Some("没有匹配的操作"),
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
        (Locale::ZhCn, "launcher.feedback.deferred.detail") => {
            Some("这个操作需要显式确认。默认测试和预览不会打开外部应用。")
        }
        (Locale::EnUs, "launcher.feedback.deferred.detail") => {
            Some("This action needs explicit confirmation. Tests and previews do not open external apps.")
        }
        (Locale::ZhCn, "launcher.feedback.failed") => Some("无法执行"),
        (Locale::EnUs, "launcher.feedback.failed") => Some("Unable to run"),
        (Locale::ZhCn, "launcher.feedback.truncated") => Some("更多内容已复制"),
        (Locale::EnUs, "launcher.feedback.truncated") => Some("more copied"),
        (Locale::ZhCn, "launcher.feedback.icon.completed") => Some("完成状态"),
        (Locale::EnUs, "launcher.feedback.icon.completed") => Some("Completed status"),
        (Locale::ZhCn, "launcher.feedback.icon.deferred") => Some("确认状态"),
        (Locale::EnUs, "launcher.feedback.icon.deferred") => Some("Review status"),
        (Locale::ZhCn, "launcher.feedback.icon.failed") => Some("错误状态"),
        (Locale::EnUs, "launcher.feedback.icon.failed") => Some("Error status"),
        (Locale::ZhCn, "launcher.feedback.action.a11y") => {
            Some("{action}，{target} 的反馈操作，状态 {status}，按 Enter")
        }
        (Locale::EnUs, "launcher.feedback.action.a11y") => {
            Some("{action}, feedback action for {target}, {status}, press Enter")
        }
        (Locale::ZhCn, "launcher.feedback.panel.a11y") => {
            Some("执行反馈，状态 {status}，操作 {target}，可用操作 {actions}")
        }
        (Locale::EnUs, "launcher.feedback.panel.a11y") => {
            Some("Execution feedback, {status}, action {target}, available actions {actions}")
        }
        _ => None,
    }
}

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    match key {
        "launcher.action.filter.hint" => Some("Filter actions"),
        "launcher.action.filter.a11y" => Some("Action Panel filter"),
        "launcher.action.filter.value.empty" => Some("empty"),
        "launcher.action.filter.input.a11y" => Some("{label}, text box, value {value}"),
        "launcher.action.row.a11y" => Some("{label} action"),
        "launcher.action.no_matches" => Some("No matching actions"),
        "launcher.feedback.copy" => Some("Copy"),
        "launcher.feedback.retry" => Some("Retry"),
        "launcher.feedback.open_studio" => Some("Open Studio"),
        "launcher.feedback.completed" => Some("Completed"),
        "launcher.feedback.deferred" => Some("Needs review"),
        "launcher.feedback.deferred.detail" => {
            Some("This action needs explicit confirmation. Tests and previews do not open external apps.")
        }
        "launcher.feedback.failed" => Some("Unable to run"),
        "launcher.feedback.truncated" => Some("more copied"),
        "launcher.feedback.icon.completed" => Some("Completed status"),
        "launcher.feedback.icon.deferred" => Some("Review status"),
        "launcher.feedback.icon.failed" => Some("Error status"),
        "launcher.feedback.action.a11y" => {
            Some("{action}, feedback action for {target}, {status}, press Enter")
        }
        "launcher.feedback.panel.a11y" => {
            Some("Execution feedback, {status}, action {target}, available actions {actions}")
        }
        _ => None,
    }
}
