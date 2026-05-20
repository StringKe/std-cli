use super::{
    color::effective_theme,
    geometry::Space,
    typography::{install_fonts_for_a11y, Text, UiScale},
    Color, EffectiveTheme, ThemeMode,
};
use crate::a11y::AccessibilityContext;
use egui::{Stroke, TextStyle};

pub fn apply_theme(ctx: &egui::Context, mode: ThemeMode) {
    let a11y = AccessibilityContext::from_env();
    apply_theme_with_scale(ctx, mode, UiScale::from_env(), &a11y);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThemeSmokeReport {
    pub dark_panel: egui::Color32,
    pub light_panel: egui::Color32,
    pub dark_window: egui::Color32,
    pub light_window: egui::Color32,
    pub dark_mode_applied: bool,
    pub light_mode_applied: bool,
}

impl ThemeSmokeReport {
    pub fn new() -> Self {
        let ctx = egui::Context::default();
        apply_theme(&ctx, ThemeMode::Dark);
        let dark = ctx.style().visuals.clone();
        apply_theme(&ctx, ThemeMode::Light);
        let light = ctx.style().visuals.clone();
        Self {
            dark_panel: dark.panel_fill,
            light_panel: light.panel_fill,
            dark_window: dark.window_fill,
            light_window: light.window_fill,
            dark_mode_applied: dark.dark_mode,
            light_mode_applied: !light.dark_mode,
        }
    }

    pub fn pass(&self) -> bool {
        self.dark_mode_applied
            && self.light_mode_applied
            && self.dark_panel != self.light_panel
            && self.dark_window != self.light_window
            && self.dark_panel.r() >= 24
    }

    pub fn summary(&self, surface: &str) -> String {
        format!(
            "{surface}_theme_smoke {}\ndark_panel={}\nlight_panel={}\ndark_window={}\nlight_window={}\ndark_mode_applied={}\nlight_mode_applied={}",
            if self.pass() { "PASS" } else { "FAIL" },
            color_hex(self.dark_panel),
            color_hex(self.light_panel),
            color_hex(self.dark_window),
            color_hex(self.light_window),
            self.dark_mode_applied,
            self.light_mode_applied
        )
    }
}

impl Default for ThemeSmokeReport {
    fn default() -> Self {
        Self::new()
    }
}

fn color_hex(color: egui::Color32) -> String {
    format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b())
}

fn apply_theme_with_scale(
    ctx: &egui::Context,
    mode: ThemeMode,
    scale: UiScale,
    a11y: &AccessibilityContext,
) {
    install_fonts_for_a11y(ctx, a11y);
    let effective = effective_theme(ctx, mode);
    let mut visuals = match effective {
        EffectiveTheme::Dark => egui::Visuals::dark(),
        EffectiveTheme::Light => egui::Visuals::light(),
    };
    visuals.panel_fill = Color::bg_surface_0_for(effective, a11y);
    visuals.window_fill = Color::bg_surface_1_for(effective, a11y);
    visuals.extreme_bg_color = Color::bg_surface_0_for(effective, a11y);
    visuals.faint_bg_color = Color::bg_surface_1_for(effective, a11y);
    visuals.widgets.noninteractive.fg_stroke =
        Stroke::new(1.0, Color::fg_primary_for(effective, a11y));
    visuals.widgets.inactive.bg_fill = Color::bg_surface_2_for(effective, a11y);
    visuals.widgets.hovered.bg_fill = Color::bg_surface_3_for(effective, a11y);
    visuals.widgets.active.bg_fill = Color::accent_weak_for(effective, a11y);
    visuals.selection.bg_fill = Color::accent_weak_for(effective, a11y);
    visuals.selection.stroke = Stroke::new(1.0, Color::accent_base_for(effective, a11y));
    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style
        .text_styles
        .insert(TextStyle::Small, Text::caption_for_scale(scale));
    style
        .text_styles
        .insert(TextStyle::Body, Text::body_for_scale(scale));
    style
        .text_styles
        .insert(TextStyle::Button, Text::body_for_scale(scale));
    style
        .text_styles
        .insert(TextStyle::Heading, Text::headline_for_scale(scale));
    style
        .text_styles
        .insert(TextStyle::Monospace, Text::code_for_scale(scale));
    style.spacing.item_spacing =
        egui::vec2(scale.f32(Space::XS as f32), scale.f32(Space::TWO_XS as f32));
    style.spacing.button_padding =
        egui::vec2(scale.f32(Space::XS as f32), scale.f32(Space::TWO_XS as f32));
    style.spacing.menu_margin = egui::Margin::same(scale.i8(Space::XS));
    style.spacing.indent = Space::md_for_scale(scale);
    ctx.set_style(style);
}

pub fn ime_composing(ctx: &egui::Context) -> bool {
    crate::input::ime_composing(ctx)
}

pub fn reduce_motion() -> bool {
    std::env::var("STD_REDUCE_MOTION")
        .or_else(|_| std::env::var("STDCLI_REDUCE_MOTION"))
        .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "on"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::palette;

    #[test]
    fn apply_theme_sets_expected_visual_mode() {
        let ctx = egui::Context::default();
        apply_theme(&ctx, ThemeMode::Light);
        assert!(!ctx.style().visuals.dark_mode);
        apply_theme(&ctx, ThemeMode::Dark);
        assert!(ctx.style().visuals.dark_mode);
    }

    #[test]
    fn apply_theme_uses_requested_mode_for_surface_tokens() {
        let ctx = egui::Context::default();

        apply_theme(&ctx, ThemeMode::Dark);
        apply_theme(&ctx, ThemeMode::Light);

        let visuals = &ctx.style().visuals;
        assert_eq!(visuals.panel_fill, palette::LIGHT_SURFACE_0);
        assert_eq!(visuals.window_fill, palette::LIGHT_SURFACE_1);
        assert_eq!(visuals.selection.bg_fill, palette::LIGHT_ACCENT_WEAK);
    }

    #[test]
    fn apply_theme_uses_non_black_dark_surface_tokens() {
        let ctx = egui::Context::default();

        apply_theme(&ctx, ThemeMode::Dark);

        let visuals = &ctx.style().visuals;
        assert_eq!(visuals.panel_fill, palette::DARK_SURFACE_0);
        assert_eq!(visuals.window_fill, palette::DARK_SURFACE_1);
        assert!(visuals.panel_fill.r() >= 24);
    }

    #[test]
    fn apply_theme_applies_zoom_to_egui_text_styles() {
        let ctx = egui::Context::default();
        let a11y = AccessibilityContext::from_env();

        apply_theme_with_scale(&ctx, ThemeMode::Dark, UiScale::new(1.5), &a11y);

        assert_eq!(ctx.style().text_styles[&TextStyle::Body].size, 19.5);
        assert_eq!(ctx.style().text_styles[&TextStyle::Heading].size, 27.0);
        assert_eq!(ctx.style().spacing.item_spacing.x, 12.0);
        assert_eq!(ctx.style().spacing.indent, 24.0);
    }

    #[test]
    fn theme_smoke_report_covers_forced_light_and_dark_tokens() {
        let report = ThemeSmokeReport::new();

        assert!(report.pass());
        assert!(report
            .summary("launcher")
            .contains("launcher_theme_smoke PASS"));
        assert!(report.summary("studio").contains("studio_theme_smoke PASS"));
        assert_eq!(color_hex(report.dark_panel), "#1C1E22");
        assert_eq!(color_hex(report.light_panel), "#FFFFFF");
    }
}
