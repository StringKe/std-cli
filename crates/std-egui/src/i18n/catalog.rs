mod launcher;
mod studio;

use super::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
    launcher::translate(locale, key).or_else(|| studio::translate(locale, key))
}

pub(super) fn fallback(key: &str) -> &'static str {
    launcher::fallback(key)
        .or_else(|| studio::fallback(key))
        .unwrap_or("UNKNOWN_I18N_KEY")
}
