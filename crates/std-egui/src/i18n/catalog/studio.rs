mod analysis;
mod apps;
mod dashboard;
mod history;
mod memory;
mod operations;
mod plugins;
mod settings;
mod windows;
mod workflow_builder;
mod workflows;

use crate::i18n::Locale;

pub(super) fn translate(locale: Locale, key: &str) -> Option<&'static str> {
    analysis::translate(locale, key)
        .or_else(|| apps::translate(locale, key))
        .or_else(|| settings::translate(locale, key))
        .or_else(|| dashboard::translate(locale, key))
        .or_else(|| history::translate(locale, key))
        .or_else(|| memory::translate(locale, key))
        .or_else(|| operations::translate(locale, key))
        .or_else(|| plugins::translate(locale, key))
        .or_else(|| workflow_builder::translate(locale, key))
        .or_else(|| windows::translate(locale, key))
        .or_else(|| workflows::translate(locale, key))
}

pub(super) fn fallback(key: &str) -> Option<&'static str> {
    analysis::fallback(key)
        .or_else(|| apps::fallback(key))
        .or_else(|| settings::fallback(key))
        .or_else(|| dashboard::fallback(key))
        .or_else(|| history::fallback(key))
        .or_else(|| memory::fallback(key))
        .or_else(|| operations::fallback(key))
        .or_else(|| plugins::fallback(key))
        .or_else(|| workflow_builder::fallback(key))
        .or_else(|| windows::fallback(key))
        .or_else(|| workflows::fallback(key))
}
