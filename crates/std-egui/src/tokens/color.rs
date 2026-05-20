use crate::a11y::AccessibilityContext;
use egui::Color32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    System,
    Dark,
    Light,
}

impl ThemeMode {
    pub fn resolve(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "dark" => Self::Dark,
            "light" => Self::Light,
            _ => Self::System,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectiveTheme {
    Dark,
    Light,
}

pub struct Color;

impl Color {
    pub fn bg_surface_0(ctx: &egui::Context) -> Color32 {
        themed(ctx, Self::bg_surface_0_for)
    }

    pub(crate) fn bg_surface_0_for(theme: EffectiveTheme, _a11y: &AccessibilityContext) -> Color32 {
        color_for(
            theme,
            Color32::from_rgb(14, 15, 17),
            Color32::from_rgb(255, 255, 255),
        )
    }

    pub fn bg_surface_1(ctx: &egui::Context) -> Color32 {
        themed(ctx, Self::bg_surface_1_for)
    }

    pub(crate) fn bg_surface_1_for(theme: EffectiveTheme, _a11y: &AccessibilityContext) -> Color32 {
        color_for(
            theme,
            Color32::from_rgb(22, 24, 27),
            Color32::from_rgb(247, 248, 250),
        )
    }

    pub fn bg_surface_2(ctx: &egui::Context) -> Color32 {
        themed(ctx, Self::bg_surface_2_for)
    }

    pub(crate) fn bg_surface_2_for(theme: EffectiveTheme, _a11y: &AccessibilityContext) -> Color32 {
        color_for(
            theme,
            Color32::from_rgb(30, 33, 38),
            Color32::from_rgb(238, 240, 243),
        )
    }

    pub fn bg_surface_3(ctx: &egui::Context) -> Color32 {
        themed(ctx, Self::bg_surface_3_for)
    }

    pub(crate) fn bg_surface_3_for(theme: EffectiveTheme, _a11y: &AccessibilityContext) -> Color32 {
        color_for(
            theme,
            Color32::from_rgb(38, 42, 48),
            Color32::from_rgb(227, 230, 234),
        )
    }

    pub fn fg_primary(ctx: &egui::Context) -> Color32 {
        themed(ctx, Self::fg_primary_for)
    }

    pub(crate) fn fg_primary_for(theme: EffectiveTheme, _a11y: &AccessibilityContext) -> Color32 {
        color_for(
            theme,
            Color32::from_rgb(236, 238, 241),
            Color32::from_rgb(26, 28, 32),
        )
    }

    pub fn fg_secondary(ctx: &egui::Context) -> Color32 {
        themed(ctx, Self::fg_secondary_for)
    }

    pub(crate) fn fg_secondary_for(theme: EffectiveTheme, a11y: &AccessibilityContext) -> Color32 {
        if a11y.high_contrast {
            return color_for(
                theme,
                Color32::from_rgb(220, 223, 227),
                Color32::from_rgb(42, 45, 50),
            );
        }
        color_for(
            theme,
            Color32::from_rgb(181, 186, 193),
            Color32::from_rgb(75, 80, 87),
        )
    }

    pub fn fg_tertiary(ctx: &egui::Context) -> Color32 {
        themed(ctx, Self::fg_tertiary_for)
    }

    pub(crate) fn fg_tertiary_for(theme: EffectiveTheme, a11y: &AccessibilityContext) -> Color32 {
        if a11y.high_contrast {
            return color_for(
                theme,
                Color32::from_rgb(181, 186, 193),
                Color32::from_rgb(75, 80, 87),
            );
        }
        Color32::from_rgb(122, 128, 137)
    }

    pub fn stroke_divider(ctx: &egui::Context) -> Color32 {
        themed(ctx, Self::stroke_divider_for)
    }

    pub(crate) fn stroke_divider_for(
        theme: EffectiveTheme,
        a11y: &AccessibilityContext,
    ) -> Color32 {
        if a11y.high_contrast {
            return Self::stroke_border_for(theme, a11y);
        }
        color_for(
            theme,
            Color32::from_rgb(38, 42, 48),
            Color32::from_rgb(227, 230, 234),
        )
    }

    pub fn stroke_border(ctx: &egui::Context) -> Color32 {
        themed(ctx, Self::stroke_border_for)
    }

    pub(crate) fn stroke_border_for(
        theme: EffectiveTheme,
        _a11y: &AccessibilityContext,
    ) -> Color32 {
        color_for(
            theme,
            Color32::from_rgb(52, 57, 63),
            Color32::from_rgb(208, 212, 217),
        )
    }

    pub fn accent_base(ctx: &egui::Context) -> Color32 {
        themed(ctx, Self::accent_base_for)
    }

    pub(crate) fn accent_base_for(theme: EffectiveTheme, _a11y: &AccessibilityContext) -> Color32 {
        color_for(
            theme,
            Color32::from_rgb(78, 156, 255),
            Color32::from_rgb(10, 107, 255),
        )
    }

    pub fn accent_weak(ctx: &egui::Context) -> Color32 {
        themed(ctx, Self::accent_weak_for)
    }

    pub(crate) fn accent_weak_for(theme: EffectiveTheme, a11y: &AccessibilityContext) -> Color32 {
        if a11y.high_contrast {
            return color_for(
                theme,
                Color32::from_rgba_premultiplied(78, 156, 255, 82),
                Color32::from_rgba_premultiplied(10, 107, 255, 56),
            );
        }
        color_for(
            theme,
            Color32::from_rgba_premultiplied(78, 156, 255, 46),
            Color32::from_rgba_premultiplied(10, 107, 255, 31),
        )
    }

    pub fn status_success(ctx: &egui::Context) -> Color32 {
        themed(ctx, Self::status_success_for)
    }

    pub(crate) fn status_success_for(
        theme: EffectiveTheme,
        _a11y: &AccessibilityContext,
    ) -> Color32 {
        color_for(
            theme,
            Color32::from_rgb(61, 203, 124),
            Color32::from_rgb(19, 135, 80),
        )
    }

    pub fn status_warning(ctx: &egui::Context) -> Color32 {
        themed(ctx, Self::status_warning_for)
    }

    pub(crate) fn status_warning_for(
        theme: EffectiveTheme,
        _a11y: &AccessibilityContext,
    ) -> Color32 {
        color_for(
            theme,
            Color32::from_rgb(245, 182, 67),
            Color32::from_rgb(178, 117, 0),
        )
    }

    pub fn status_danger(ctx: &egui::Context) -> Color32 {
        themed(ctx, Self::status_danger_for)
    }

    pub(crate) fn status_danger_for(
        theme: EffectiveTheme,
        _a11y: &AccessibilityContext,
    ) -> Color32 {
        color_for(
            theme,
            Color32::from_rgb(255, 106, 106),
            Color32::from_rgb(200, 49, 43),
        )
    }
}

pub(crate) fn effective_theme(ctx: &egui::Context, mode: ThemeMode) -> EffectiveTheme {
    match mode {
        ThemeMode::Dark => EffectiveTheme::Dark,
        ThemeMode::Light => EffectiveTheme::Light,
        ThemeMode::System if ctx.style().visuals.dark_mode => EffectiveTheme::Dark,
        ThemeMode::System => EffectiveTheme::Light,
    }
}

fn themed(
    ctx: &egui::Context,
    color: fn(EffectiveTheme, &AccessibilityContext) -> Color32,
) -> Color32 {
    let a11y = AccessibilityContext::from_env();
    color(effective_theme(ctx, ThemeMode::System), &a11y)
}

fn color_for(theme: EffectiveTheme, dark: Color32, light: Color32) -> Color32 {
    match theme {
        EffectiveTheme::Dark => dark,
        EffectiveTheme::Light => light,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn a11y(high_contrast: bool) -> AccessibilityContext {
        AccessibilityContext {
            reduce_motion: false,
            reduce_transparency: false,
            high_contrast,
            bold_text: false,
        }
    }

    #[test]
    fn theme_mode_resolves_config_values() {
        assert_eq!(ThemeMode::resolve("dark"), ThemeMode::Dark);
        assert_eq!(ThemeMode::resolve("light"), ThemeMode::Light);
        assert_eq!(ThemeMode::resolve("system"), ThemeMode::System);
        assert_eq!(ThemeMode::resolve("unknown"), ThemeMode::System);
    }

    #[test]
    fn high_contrast_uses_documented_color_overrides() {
        let standard = a11y(false);
        let high_contrast = a11y(true);

        assert_eq!(
            Color::fg_secondary_for(EffectiveTheme::Dark, &high_contrast),
            Color32::from_rgb(220, 223, 227)
        );
        assert_eq!(
            Color::fg_secondary_for(EffectiveTheme::Light, &high_contrast),
            Color32::from_rgb(42, 45, 50)
        );
        assert_ne!(
            Color::accent_weak_for(EffectiveTheme::Dark, &standard),
            Color::accent_weak_for(EffectiveTheme::Dark, &high_contrast)
        );
    }
}
