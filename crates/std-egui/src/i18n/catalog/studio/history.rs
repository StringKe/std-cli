use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
        (Locale::ZhCn, "studio.history.title") => Some("History"),
        (Locale::EnUs, "studio.history.title") => Some("History"),
        (Locale::ZhCn, "studio.history.detail") => Some("workflow trace 与 audit event"),
        (Locale::EnUs, "studio.history.detail") => Some("workflow traces and audit events"),
        (Locale::ZhCn, "studio.history.traces.title") => Some("Workflow Traces"),
        (Locale::EnUs, "studio.history.traces.title") => Some("Workflow Traces"),
        (Locale::ZhCn, "studio.history.traces.detail") => Some("已持久化执行时间线"),
        (Locale::EnUs, "studio.history.traces.detail") => Some("persisted execution timeline"),
        (Locale::ZhCn, "studio.history.traces.empty") => Some("没有 workflow trace"),
        (Locale::EnUs, "studio.history.traces.empty") => Some("No workflow traces"),
        (Locale::ZhCn, "studio.history.events.title") => Some("Audit Events"),
        (Locale::EnUs, "studio.history.events.title") => Some("Audit Events"),
        (Locale::ZhCn, "studio.history.events.detail") => Some("最近 core 事件日志"),
        (Locale::EnUs, "studio.history.events.detail") => Some("recent core event log"),
        (Locale::ZhCn, "studio.history.events.empty") => Some("没有 audit event"),
        (Locale::EnUs, "studio.history.events.empty") => Some("No audit events"),
        _ => None,
    }
}

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    match key {
        "studio.history.title" => Some("History"),
        "studio.history.detail" => Some("workflow traces and audit events"),
        "studio.history.traces.title" => Some("Workflow Traces"),
        "studio.history.traces.detail" => Some("persisted execution timeline"),
        "studio.history.traces.empty" => Some("No workflow traces"),
        "studio.history.events.title" => Some("Audit Events"),
        "studio.history.events.detail" => Some("recent core event log"),
        "studio.history.events.empty" => Some("No audit events"),
        _ => None,
    }
}
