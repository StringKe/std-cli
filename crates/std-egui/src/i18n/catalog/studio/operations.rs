use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
        (Locale::ZhCn, "studio.operations.title") => Some("Operations"),
        (Locale::EnUs, "studio.operations.title") => Some("Operations"),
        (Locale::ZhCn, "studio.operations.detail") => Some("QA、Doctor、Release、Install、Runtime"),
        (Locale::EnUs, "studio.operations.detail") => Some("QA, Doctor, Release, Install, Runtime"),
        (Locale::ZhCn, "studio.operations.command") => Some("命令"),
        (Locale::EnUs, "studio.operations.command") => Some("Command"),
        (Locale::ZhCn, "studio.operations.runbook") => Some("执行链"),
        (Locale::EnUs, "studio.operations.runbook") => Some("Runbook"),
        (Locale::ZhCn, "studio.operations.evidence") => Some("证据"),
        (Locale::EnUs, "studio.operations.evidence") => Some("Evidence"),
        (Locale::ZhCn, "studio.operations.result") => Some("结果"),
        (Locale::EnUs, "studio.operations.result") => Some("Result"),
        (Locale::ZhCn, "studio.operations.artifact") => Some("产物"),
        (Locale::EnUs, "studio.operations.artifact") => Some("Artifact"),
        (Locale::ZhCn, "studio.operations.output") => Some("输出"),
        (Locale::EnUs, "studio.operations.output") => Some("Output"),
        (Locale::ZhCn, "studio.operations.current_workspace") => Some("当前 workspace 状态"),
        (Locale::EnUs, "studio.operations.current_workspace") => Some("current workspace state"),
        (Locale::ZhCn, "studio.operations.record_evidence") => Some("记录证据"),
        (Locale::EnUs, "studio.operations.record_evidence") => Some("Record Evidence"),
        (Locale::ZhCn, "studio.operations.workspace_policy.title") => Some("Workspace Policy"),
        (Locale::EnUs, "studio.operations.workspace_policy.title") => Some("Workspace Policy"),
        (Locale::ZhCn, "studio.operations.workspace_policy.detail") => {
            Some("单宿主窗口和内部 pane")
        }
        (Locale::EnUs, "studio.operations.workspace_policy.detail") => {
            Some("single host window and internal panes")
        }
        (Locale::ZhCn, "studio.operations.workspace_policy.host") => Some("Host"),
        (Locale::EnUs, "studio.operations.workspace_policy.host") => Some("Host"),
        (Locale::ZhCn, "studio.operations.workspace_policy.host.detail") => {
            Some("自绘 egui host chrome")
        }
        (Locale::EnUs, "studio.operations.workspace_policy.host.detail") => {
            Some("egui-rendered host chrome")
        }
        (Locale::ZhCn, "studio.operations.workspace_policy.panes") => Some("Panes"),
        (Locale::EnUs, "studio.operations.workspace_policy.panes") => Some("Panes"),
        (Locale::ZhCn, "studio.operations.workspace_policy.panes.detail") => {
            Some("工作台内部状态对象")
        }
        (Locale::EnUs, "studio.operations.workspace_policy.panes.detail") => {
            Some("internal workspace state objects")
        }
        (Locale::ZhCn, "studio.operations.workspace_policy.native") => Some("Native child windows"),
        (Locale::EnUs, "studio.operations.workspace_policy.native") => Some("Native child windows"),
        (Locale::ZhCn, "studio.operations.workspace_policy.native.detail") => {
            Some("主路径禁止原生子窗口")
        }
        (Locale::EnUs, "studio.operations.workspace_policy.native.detail") => {
            Some("forbidden on main path")
        }
        (Locale::ZhCn, "studio.operations.workspace_policy.detached") => Some("Detached panels"),
        (Locale::EnUs, "studio.operations.workspace_policy.detached") => Some("Detached panels"),
        (Locale::ZhCn, "studio.operations.workspace_policy.detached.detail") => {
            Some("主路径禁止游离面板")
        }
        (Locale::EnUs, "studio.operations.workspace_policy.detached.detail") => {
            Some("forbidden on main path")
        }
        (Locale::ZhCn, "studio.operations.workspace_policy.docs") => Some("Docs"),
        (Locale::EnUs, "studio.operations.workspace_policy.docs") => Some("Docs"),
        (Locale::ZhCn, "studio.operations.workspace_policy.docs.detail") => Some("UI 单一真相源"),
        (Locale::EnUs, "studio.operations.workspace_policy.docs.detail") => {
            Some("single UI source of truth")
        }
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
        "studio.operations.runbook" => Some("Runbook"),
        "studio.operations.evidence" => Some("Evidence"),
        "studio.operations.result" => Some("Result"),
        "studio.operations.artifact" => Some("Artifact"),
        "studio.operations.output" => Some("Output"),
        "studio.operations.current_workspace" => Some("current workspace state"),
        "studio.operations.record_evidence" => Some("Record Evidence"),
        "studio.operations.workspace_policy.title" => Some("Workspace Policy"),
        "studio.operations.workspace_policy.detail" => {
            Some("single host window and internal panes")
        }
        "studio.operations.workspace_policy.host" => Some("Host"),
        "studio.operations.workspace_policy.host.detail" => Some("egui-rendered host chrome"),
        "studio.operations.workspace_policy.panes" => Some("Panes"),
        "studio.operations.workspace_policy.panes.detail" => {
            Some("internal workspace state objects")
        }
        "studio.operations.workspace_policy.native" => Some("Native child windows"),
        "studio.operations.workspace_policy.native.detail" => Some("forbidden on main path"),
        "studio.operations.workspace_policy.detached" => Some("Detached panels"),
        "studio.operations.workspace_policy.detached.detail" => Some("forbidden on main path"),
        "studio.operations.workspace_policy.docs" => Some("Docs"),
        "studio.operations.workspace_policy.docs.detail" => Some("single UI source of truth"),
        "studio.operations.completion.title" => Some("Completion Audit"),
        "studio.operations.completion.detail" => Some("not complete until proven"),
        "studio.operations.completion.note" => {
            Some("Each area requires current runtime evidence before completion.")
        }
        _ => None,
    }
}
