use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
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

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    match key {
        "studio.operations.title" => Some("Operations"),
        "studio.operations.detail" => Some("QA, Doctor, Release, Install, Runtime"),
        "studio.operations.command" => Some("Command"),
        "studio.operations.evidence" => Some("Evidence"),
        "studio.operations.current_workspace" => Some("current workspace state"),
        "studio.operations.record_evidence" => Some("Record Evidence"),
        "studio.operations.completion.title" => Some("Completion Audit"),
        "studio.operations.completion.detail" => Some("not complete until proven"),
        "studio.operations.completion.note" => {
            Some("Each area requires current runtime evidence before completion.")
        }
        _ => None,
    }
}
