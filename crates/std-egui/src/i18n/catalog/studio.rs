mod dashboard;
mod operations;
mod settings;
mod workflows;

use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
    settings::translate(locale, key)
        .or_else(|| dashboard::translate(locale, key))
        .or_else(|| operations::translate(locale, key))
        .or_else(|| workflows::translate(locale, key))
}

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    settings::fallback(key)
        .or_else(|| dashboard::fallback(key))
        .or_else(|| operations::fallback(key))
        .or_else(|| workflows::fallback(key))
}
