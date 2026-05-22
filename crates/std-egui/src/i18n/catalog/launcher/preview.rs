use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
    match (locale, key) {
        (Locale::ZhCn, "launcher.preview.title") => Some("预览"),
        (Locale::EnUs, "launcher.preview.title") => Some("Preview"),
        (Locale::ZhCn, "launcher.preview.examples") => Some("示例"),
        (Locale::EnUs, "launcher.preview.examples") => Some("Examples"),
        (Locale::ZhCn, "launcher.preview.a11y") => Some("预览，{title}，{command}"),
        (Locale::EnUs, "launcher.preview.a11y") => Some("Preview, {title}, {command}"),
        _ => None,
    }
}

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    match key {
        "launcher.preview.title" => Some("Preview"),
        "launcher.preview.examples" => Some("Examples"),
        "launcher.preview.a11y" => Some("Preview, {title}, {command}"),
        _ => None,
    }
}
