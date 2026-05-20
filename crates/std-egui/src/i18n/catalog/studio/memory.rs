use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
        (Locale::ZhCn, "studio.memory.title") => Some("Memory Browser"),
        (Locale::EnUs, "studio.memory.title") => Some("Memory Browser"),
        (Locale::ZhCn, "studio.memory.detail") => Some("搜索、检查、写入 std-core 存储"),
        (Locale::EnUs, "studio.memory.detail") => {
            Some("search, inspect, write through std-core storage")
        }
        (Locale::ZhCn, "studio.memory.search.hint") => Some("title、body、tag、scope"),
        (Locale::EnUs, "studio.memory.search.hint") => Some("title, body, tag, scope"),
        (Locale::ZhCn, "studio.memory.search") => Some("搜索"),
        (Locale::EnUs, "studio.memory.search") => Some("Search"),
        (Locale::ZhCn, "studio.memory.records.title") => Some("Records"),
        (Locale::EnUs, "studio.memory.records.title") => Some("Records"),
        (Locale::ZhCn, "studio.memory.records.detail") => Some("本地 recall 结果"),
        (Locale::EnUs, "studio.memory.records.detail") => Some("local recall results"),
        (Locale::ZhCn, "studio.memory.records.empty") => Some("没有 memory 记录"),
        (Locale::EnUs, "studio.memory.records.empty") => Some("No memory records"),
        (Locale::ZhCn, "studio.memory.detail.title") => Some("Detail"),
        (Locale::EnUs, "studio.memory.detail.title") => Some("Detail"),
        (Locale::ZhCn, "studio.memory.detail.detail") => Some("已选择 memory"),
        (Locale::EnUs, "studio.memory.detail.detail") => Some("selected memory"),
        (Locale::ZhCn, "studio.memory.detail.empty") => Some("选择一条 memory"),
        (Locale::EnUs, "studio.memory.detail.empty") => Some("Select a memory"),
        (Locale::ZhCn, "studio.memory.write.title") => Some("Write"),
        (Locale::EnUs, "studio.memory.write.title") => Some("Write"),
        (Locale::ZhCn, "studio.memory.write.detail") => Some("持久化新上下文"),
        (Locale::EnUs, "studio.memory.write.detail") => Some("persist new context"),
        (Locale::ZhCn, "studio.memory.scope") => Some("Scope"),
        (Locale::EnUs, "studio.memory.scope") => Some("Scope"),
        (Locale::ZhCn, "studio.memory.item_title") => Some("Title"),
        (Locale::EnUs, "studio.memory.item_title") => Some("Title"),
        (Locale::ZhCn, "studio.memory.body") => Some("Body"),
        (Locale::EnUs, "studio.memory.body") => Some("Body"),
        (Locale::ZhCn, "studio.memory.tags") => Some("Tags"),
        (Locale::EnUs, "studio.memory.tags") => Some("Tags"),
        (Locale::ZhCn, "studio.memory.remember") => Some("Remember"),
        (Locale::EnUs, "studio.memory.remember") => Some("Remember"),
        (Locale::ZhCn, "studio.memory.clear") => Some("Clear"),
        (Locale::EnUs, "studio.memory.clear") => Some("Clear"),
        _ => None,
    }
}

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    match key {
        "studio.memory.title" => Some("Memory Browser"),
        "studio.memory.detail" => Some("search, inspect, write through std-core storage"),
        "studio.memory.search.hint" => Some("title, body, tag, scope"),
        "studio.memory.search" => Some("Search"),
        "studio.memory.records.title" => Some("Records"),
        "studio.memory.records.detail" => Some("local recall results"),
        "studio.memory.records.empty" => Some("No memory records"),
        "studio.memory.detail.title" => Some("Detail"),
        "studio.memory.detail.detail" => Some("selected memory"),
        "studio.memory.detail.empty" => Some("Select a memory"),
        "studio.memory.write.title" => Some("Write"),
        "studio.memory.write.detail" => Some("persist new context"),
        "studio.memory.scope" => Some("Scope"),
        "studio.memory.item_title" => Some("Title"),
        "studio.memory.body" => Some("Body"),
        "studio.memory.tags" => Some("Tags"),
        "studio.memory.remember" => Some("Remember"),
        "studio.memory.clear" => Some("Clear"),
        _ => None,
    }
}
