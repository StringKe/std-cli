use super::{palette, Color, EffectiveTheme};
use crate::a11y::AccessibilityContext;
use egui::Color32;

pub(crate) fn min_text_contrast_ratio() -> f32 {
    let standard = AccessibilityContext {
        reduce_motion: false,
        reduce_transparency: false,
        high_contrast: false,
        bold_text: false,
    };
    [
        text_contrast_for(EffectiveTheme::Dark, &standard),
        text_contrast_for(EffectiveTheme::Light, &standard),
    ]
    .into_iter()
    .fold(f32::MAX, f32::min)
}

pub(crate) fn min_secondary_text_contrast_ratio() -> f32 {
    min_token_text_contrast_ratio(false, Color::fg_secondary_for)
}

pub(crate) fn min_tertiary_text_contrast_ratio() -> f32 {
    min_token_text_contrast_ratio(false, Color::fg_tertiary_for)
}

pub(crate) fn min_high_contrast_secondary_text_contrast_ratio() -> f32 {
    min_token_text_contrast_ratio(true, Color::fg_secondary_for)
}

pub(crate) fn min_accent_nontext_contrast_ratio() -> f32 {
    let standard = AccessibilityContext {
        reduce_motion: false,
        reduce_transparency: false,
        high_contrast: false,
        bold_text: false,
    };
    [
        contrast_ratio(
            Color::accent_base_for(EffectiveTheme::Dark, &standard),
            palette::DARK_SURFACE_0,
        ),
        contrast_ratio(
            Color::accent_base_for(EffectiveTheme::Light, &standard),
            palette::LIGHT_SURFACE_0,
        ),
    ]
    .into_iter()
    .fold(f32::MAX, f32::min)
}

fn min_token_text_contrast_ratio(
    high_contrast: bool,
    token: fn(EffectiveTheme, &AccessibilityContext) -> Color32,
) -> f32 {
    let a11y = AccessibilityContext {
        reduce_motion: false,
        reduce_transparency: false,
        high_contrast,
        bold_text: false,
    };
    [EffectiveTheme::Dark, EffectiveTheme::Light]
        .into_iter()
        .map(|theme| contrast_ratio(token(theme, &a11y), surface_0(theme)))
        .fold(f32::MAX, f32::min)
}

fn text_contrast_for(theme: EffectiveTheme, a11y: &AccessibilityContext) -> f32 {
    contrast_ratio(Color::fg_primary_for(theme, a11y), surface_0(theme))
}

fn surface_0(theme: EffectiveTheme) -> Color32 {
    match theme {
        EffectiveTheme::Dark => palette::DARK_SURFACE_0,
        EffectiveTheme::Light => palette::LIGHT_SURFACE_0,
    }
}

fn contrast_ratio(foreground: Color32, background: Color32) -> f32 {
    let lighter = relative_luminance(foreground).max(relative_luminance(background));
    let darker = relative_luminance(foreground).min(relative_luminance(background));
    (lighter + 0.05) / (darker + 0.05)
}

fn relative_luminance(color: Color32) -> f32 {
    0.2126 * linear_channel(color.r())
        + 0.7152 * linear_channel(color.g())
        + 0.0722 * linear_channel(color.b())
}

fn linear_channel(value: u8) -> f32 {
    let value = f32::from(value) / 255.0;
    if value <= 0.04045 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_and_accent_tokens_meet_wcag_aa_contrast() {
        assert!(min_text_contrast_ratio() >= 4.5);
        assert!(min_secondary_text_contrast_ratio() >= 4.5);
        assert!(min_accent_nontext_contrast_ratio() >= 3.0);
    }

    #[test]
    fn high_contrast_secondary_text_is_stronger_than_standard() {
        assert!(min_tertiary_text_contrast_ratio() >= 3.0);
        assert!(
            min_high_contrast_secondary_text_contrast_ratio() > min_secondary_text_contrast_ratio()
        );
    }
}
