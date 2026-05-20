use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
        (Locale::ZhCn, "studio.dashboard.title") => Some("Dashboard"),
        (Locale::EnUs, "studio.dashboard.title") => Some("Dashboard"),
        (Locale::ZhCn, "studio.dashboard.detail") => Some("本地自动化层的运行概览"),
        (Locale::EnUs, "studio.dashboard.detail") => {
            Some("Operational overview for the local automation layer")
        }
        (Locale::ZhCn, "studio.dashboard.actions") => Some("Action"),
        (Locale::EnUs, "studio.dashboard.actions") => Some("Actions"),
        (Locale::ZhCn, "studio.dashboard.actions.detail") => Some("可搜索单元"),
        (Locale::EnUs, "studio.dashboard.actions.detail") => Some("searchable units"),
        (Locale::ZhCn, "studio.dashboard.memory") => Some("Memory"),
        (Locale::EnUs, "studio.dashboard.memory") => Some("Memory"),
        (Locale::ZhCn, "studio.dashboard.memory.detail") => Some("本地记录"),
        (Locale::EnUs, "studio.dashboard.memory.detail") => Some("local notes"),
        (Locale::ZhCn, "studio.dashboard.audit_events") => Some("审计事件"),
        (Locale::EnUs, "studio.dashboard.audit_events") => Some("Audit Events"),
        (Locale::ZhCn, "studio.dashboard.audit_events.detail") => Some("事件轨迹"),
        (Locale::EnUs, "studio.dashboard.audit_events.detail") => Some("event trail"),
        (Locale::ZhCn, "studio.dashboard.planner.title") => Some("Planner 草稿"),
        (Locale::EnUs, "studio.dashboard.planner.title") => Some("Planner Draft"),
        (Locale::ZhCn, "studio.dashboard.planner.detail") => Some("AI 本地计划"),
        (Locale::EnUs, "studio.dashboard.planner.detail") => Some("AI local plan"),
        (Locale::ZhCn, "studio.dashboard.goal") => Some("目标"),
        (Locale::EnUs, "studio.dashboard.goal") => Some("Goal"),
        (Locale::ZhCn, "studio.dashboard.recent_memory.title") => Some("最近 Memory"),
        (Locale::EnUs, "studio.dashboard.recent_memory.title") => Some("Recent Memory"),
        (Locale::ZhCn, "studio.dashboard.recent_memory.detail") => Some("共享 core"),
        (Locale::EnUs, "studio.dashboard.recent_memory.detail") => Some("shared core"),
        (Locale::ZhCn, "studio.dashboard.recent_memory.empty") => Some("还没有 Memory 记录"),
        (Locale::EnUs, "studio.dashboard.recent_memory.empty") => Some("No memory records yet"),
        (Locale::ZhCn, "studio.dashboard.next_gates.title") => Some("下一批 gate"),
        (Locale::EnUs, "studio.dashboard.next_gates.title") => Some("Next Gates"),
        (Locale::ZhCn, "studio.dashboard.next_gates.detail") => Some("completion audit"),
        (Locale::EnUs, "studio.dashboard.next_gates.detail") => Some("completion audit"),
        (Locale::ZhCn, "studio.dashboard.gate.launcher") => Some("Launcher PASS 证据已存在"),
        (Locale::EnUs, "studio.dashboard.gate.launcher") => Some("Launcher PASS evidence exists"),
        (Locale::ZhCn, "studio.dashboard.gate.studio") => Some("Studio 真实 UI audit 进行中"),
        (Locale::EnUs, "studio.dashboard.gate.studio") => Some("Studio real UI audit ongoing"),
        (Locale::ZhCn, "studio.dashboard.gate.quality") => Some("质量工具只使用 Rust 生态"),
        (Locale::EnUs, "studio.dashboard.gate.quality") => Some("Quality rust ecosystem only"),
        _ => None,
    }
}

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    match key {
        "studio.dashboard.title" => Some("Dashboard"),
        "studio.dashboard.detail" => Some("Operational overview for the local automation layer"),
        "studio.dashboard.actions" => Some("Actions"),
        "studio.dashboard.actions.detail" => Some("searchable units"),
        "studio.dashboard.memory" => Some("Memory"),
        "studio.dashboard.memory.detail" => Some("local notes"),
        "studio.dashboard.audit_events" => Some("Audit Events"),
        "studio.dashboard.audit_events.detail" => Some("event trail"),
        "studio.dashboard.planner.title" => Some("Planner Draft"),
        "studio.dashboard.planner.detail" => Some("AI local plan"),
        "studio.dashboard.goal" => Some("Goal"),
        "studio.dashboard.recent_memory.title" => Some("Recent Memory"),
        "studio.dashboard.recent_memory.detail" => Some("shared core"),
        "studio.dashboard.recent_memory.empty" => Some("No memory records yet"),
        "studio.dashboard.next_gates.title" => Some("Next Gates"),
        "studio.dashboard.next_gates.detail" => Some("completion audit"),
        "studio.dashboard.gate.launcher" => Some("Launcher PASS evidence exists"),
        "studio.dashboard.gate.studio" => Some("Studio real UI audit ongoing"),
        "studio.dashboard.gate.quality" => Some("Quality rust ecosystem only"),
        _ => None,
    }
}
