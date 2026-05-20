use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
        (Locale::ZhCn, "studio.apps.title") => Some("Apps"),
        (Locale::EnUs, "studio.apps.title") => Some("Apps"),
        (Locale::ZhCn, "studio.apps.detail") => Some("注册 bundle、搜索别名、安全预览启动"),
        (Locale::EnUs, "studio.apps.detail") => {
            Some("register bundles, search aliases, preview safe launch")
        }
        (Locale::ZhCn, "studio.apps.register.title") => Some("注册"),
        (Locale::EnUs, "studio.apps.register.title") => Some("Register"),
        (Locale::ZhCn, "studio.apps.register.detail") => Some("复制 app bundle 到 std 存储"),
        (Locale::EnUs, "studio.apps.register.detail") => Some("copy app bundle into std storage"),
        (Locale::ZhCn, "studio.apps.bundle_path") => Some("Bundle 路径"),
        (Locale::EnUs, "studio.apps.bundle_path") => Some("Bundle path"),
        (Locale::ZhCn, "studio.apps.register") => Some("注册"),
        (Locale::EnUs, "studio.apps.register") => Some("Register"),
        (Locale::ZhCn, "studio.apps.use_fixture_app") => Some("使用测试 App"),
        (Locale::EnUs, "studio.apps.use_fixture_app") => Some("Use fixture app"),
        (Locale::ZhCn, "studio.apps.search.title") => Some("搜索"),
        (Locale::EnUs, "studio.apps.search.title") => Some("Search"),
        (Locale::ZhCn, "studio.apps.search.detail") => Some("本地化名称与 URL scheme"),
        (Locale::EnUs, "studio.apps.search.detail") => Some("localized names and URL schemes"),
        (Locale::ZhCn, "studio.apps.search") => Some("搜索"),
        (Locale::EnUs, "studio.apps.search") => Some("Search"),
        (Locale::ZhCn, "studio.apps.preview") => Some("预览"),
        (Locale::EnUs, "studio.apps.preview") => Some("Preview"),
        (Locale::ZhCn, "studio.apps.trigger") => Some("触发"),
        (Locale::EnUs, "studio.apps.trigger") => Some("Trigger"),
        (Locale::ZhCn, "studio.apps.external_defer") => {
            Some("外部启动默认返回 NeedsExternalRunner")
        }
        (Locale::EnUs, "studio.apps.external_defer") => {
            Some("external launch defaults to NeedsExternalRunner")
        }
        (Locale::ZhCn, "studio.apps.external_runner.status") => Some("NeedsExternalRunner"),
        (Locale::EnUs, "studio.apps.external_runner.status") => Some("NeedsExternalRunner"),
        (Locale::ZhCn, "studio.apps.registered.title") => Some("已注册"),
        (Locale::EnUs, "studio.apps.registered.title") => Some("Registered"),
        (Locale::ZhCn, "studio.apps.registered.detail") => Some("受管 app bundle"),
        (Locale::EnUs, "studio.apps.registered.detail") => Some("managed app bundles"),
        (Locale::ZhCn, "studio.apps.registered.empty") => Some("没有已注册 app bundle"),
        (Locale::EnUs, "studio.apps.registered.empty") => Some("No app bundles registered"),
        (Locale::ZhCn, "studio.apps.matches.empty") => Some("没有匹配的 app action"),
        (Locale::EnUs, "studio.apps.matches.empty") => Some("No matching app actions"),
        (Locale::ZhCn, "studio.apps.select") => Some("选择"),
        (Locale::EnUs, "studio.apps.select") => Some("Select"),
        _ => None,
    }
}

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    match key {
        "studio.apps.title" => Some("Apps"),
        "studio.apps.detail" => Some("register bundles, search aliases, preview safe launch"),
        "studio.apps.register.title" => Some("Register"),
        "studio.apps.register.detail" => Some("copy app bundle into std storage"),
        "studio.apps.bundle_path" => Some("Bundle path"),
        "studio.apps.register" => Some("Register"),
        "studio.apps.use_fixture_app" => Some("Use fixture app"),
        "studio.apps.search.title" => Some("Search"),
        "studio.apps.search.detail" => Some("localized names and URL schemes"),
        "studio.apps.search" => Some("Search"),
        "studio.apps.preview" => Some("Preview"),
        "studio.apps.trigger" => Some("Trigger"),
        "studio.apps.external_defer" => Some("external launch defaults to NeedsExternalRunner"),
        "studio.apps.external_runner.status" => Some("NeedsExternalRunner"),
        "studio.apps.registered.title" => Some("Registered"),
        "studio.apps.registered.detail" => Some("managed app bundles"),
        "studio.apps.registered.empty" => Some("No app bundles registered"),
        "studio.apps.matches.empty" => Some("No matching app actions"),
        "studio.apps.select" => Some("Select"),
        _ => None,
    }
}
