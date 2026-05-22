mod feedback;
mod preview;
mod results;
mod search;

use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
    search::translate(locale, key)
        .or_else(|| results::translate(locale, key))
        .or_else(|| feedback::translate(locale, key))
        .or_else(|| preview::translate(locale, key))
}

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    search::fallback(key)
        .or_else(|| results::fallback(key))
        .or_else(|| feedback::fallback(key))
        .or_else(|| preview::fallback(key))
}
