use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
        (Locale::ZhCn, "studio.analysis.title") => Some("Index Analysis"),
        (Locale::EnUs, "studio.analysis.title") => Some("Index Analysis"),
        (Locale::ZhCn, "studio.analysis.detail") => Some("四层本地理解与 QA"),
        (Locale::EnUs, "studio.analysis.detail") => Some("four-layer local understanding and QA"),
        (Locale::ZhCn, "studio.analysis.path.hint") => Some("project、file、app、workflow 路径"),
        (Locale::EnUs, "studio.analysis.path.hint") => Some("project, file, app, workflow path"),
        (Locale::ZhCn, "studio.analysis.analyze") => Some("Analyze"),
        (Locale::EnUs, "studio.analysis.analyze") => Some("Analyze"),
        (Locale::ZhCn, "studio.analysis.entity.title") => Some("Entity"),
        (Locale::EnUs, "studio.analysis.entity.title") => Some("Entity"),
        (Locale::ZhCn, "studio.analysis.entity.detail") => Some("当前 index document"),
        (Locale::EnUs, "studio.analysis.entity.detail") => Some("active index document"),
        (Locale::ZhCn, "studio.analysis.entity.empty") => Some("没有 active analysis"),
        (Locale::EnUs, "studio.analysis.entity.empty") => Some("No active analysis"),
        (Locale::ZhCn, "studio.analysis.query.title") => Some("Ask and Search"),
        (Locale::EnUs, "studio.analysis.query.title") => Some("Ask and Search"),
        (Locale::ZhCn, "studio.analysis.query.detail") => Some("已保存 index document"),
        (Locale::EnUs, "studio.analysis.query.detail") => Some("saved index documents"),
        (Locale::ZhCn, "studio.analysis.ask") => Some("Ask"),
        (Locale::EnUs, "studio.analysis.ask") => Some("Ask"),
        (Locale::ZhCn, "studio.analysis.search") => Some("Search"),
        (Locale::EnUs, "studio.analysis.search") => Some("Search"),
        (Locale::ZhCn, "studio.analysis.inspect") => Some("Inspect"),
        (Locale::EnUs, "studio.analysis.inspect") => Some("Inspect"),
        (Locale::ZhCn, "studio.analysis.answer") => Some("Answer"),
        (Locale::EnUs, "studio.analysis.answer") => Some("Answer"),
        (Locale::ZhCn, "studio.analysis.coverage.title") => Some("Coverage"),
        (Locale::EnUs, "studio.analysis.coverage.title") => Some("Coverage"),
        (Locale::ZhCn, "studio.analysis.coverage.detail") => {
            Some("overview、components、relations、history")
        }
        (Locale::EnUs, "studio.analysis.coverage.detail") => {
            Some("overview, components, relations, history")
        }
        (Locale::ZhCn, "studio.analysis.coverage.refresh") => Some("Refresh Coverage"),
        (Locale::EnUs, "studio.analysis.coverage.refresh") => Some("Refresh Coverage"),
        (Locale::ZhCn, "studio.analysis.coverage.report") => Some("Coverage Report"),
        (Locale::EnUs, "studio.analysis.coverage.report") => Some("Coverage Report"),
        (Locale::ZhCn, "studio.analysis.output.empty") => Some("没有输出"),
        (Locale::EnUs, "studio.analysis.output.empty") => Some("No output"),
        _ => None,
    }
}

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    match key {
        "studio.analysis.title" => Some("Index Analysis"),
        "studio.analysis.detail" => Some("four-layer local understanding and QA"),
        "studio.analysis.path.hint" => Some("project, file, app, workflow path"),
        "studio.analysis.analyze" => Some("Analyze"),
        "studio.analysis.entity.title" => Some("Entity"),
        "studio.analysis.entity.detail" => Some("active index document"),
        "studio.analysis.entity.empty" => Some("No active analysis"),
        "studio.analysis.query.title" => Some("Ask and Search"),
        "studio.analysis.query.detail" => Some("saved index documents"),
        "studio.analysis.ask" => Some("Ask"),
        "studio.analysis.search" => Some("Search"),
        "studio.analysis.inspect" => Some("Inspect"),
        "studio.analysis.answer" => Some("Answer"),
        "studio.analysis.coverage.title" => Some("Coverage"),
        "studio.analysis.coverage.detail" => Some("overview, components, relations, history"),
        "studio.analysis.coverage.refresh" => Some("Refresh Coverage"),
        "studio.analysis.coverage.report" => Some("Coverage Report"),
        "studio.analysis.output.empty" => Some("No output"),
        _ => None,
    }
}
