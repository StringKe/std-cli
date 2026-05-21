use super::{
    color::effective_theme,
    geometry::Space,
    typography::{install_fonts_for_a11y, Text, UiScale},
    Color, EffectiveTheme, ThemeMode,
};
use crate::{a11y::AccessibilityContext, motion::MotionContext};
use egui::{Stroke, TextStyle};
use std::time::Duration;

pub fn apply_theme(ctx: &egui::Context, mode: ThemeMode) {
    let a11y = AccessibilityContext::from_env();
    apply_theme_with_scale(ctx, mode, UiScale::from_env(), &a11y);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThemeProfile {
    pub requested: ThemeMode,
    pub effective: EffectiveTheme,
    pub high_contrast: bool,
    pub reduce_motion: bool,
    pub focus_ring_width: u32,
}

impl ThemeProfile {
    pub fn apply(ctx: &egui::Context, mode: ThemeMode) -> Self {
        let a11y = AccessibilityContext::from_env();
        apply_theme_with_scale(ctx, mode, UiScale::from_env(), &a11y);
        Self::from_applied(ctx, mode, &a11y)
    }

    fn from_applied(ctx: &egui::Context, mode: ThemeMode, a11y: &AccessibilityContext) -> Self {
        Self {
            requested: mode,
            effective: effective_theme(ctx, mode),
            high_contrast: a11y.high_contrast,
            reduce_motion: MotionContext::from_env().is_reduced(),
            focus_ring_width: a11y.focus_ring_width() as u32,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ThemeSmokeReport {
    pub dark_surface_0: egui::Color32,
    pub dark_surface_1: egui::Color32,
    pub dark_surface_2: egui::Color32,
    pub dark_surface_3: egui::Color32,
    pub light_surface_0: egui::Color32,
    pub light_surface_1: egui::Color32,
    pub light_surface_2: egui::Color32,
    pub light_surface_3: egui::Color32,
    pub dark_selection: egui::Color32,
    pub light_selection: egui::Color32,
    pub dark_accent: egui::Color32,
    pub light_accent: egui::Color32,
    pub dark_accent_weak_alpha: u8,
    pub light_accent_weak_alpha: u8,
    pub dark_mode_applied: bool,
    pub light_mode_applied: bool,
    pub high_contrast_focus_ring_width: u32,
    pub min_text_contrast_x100: u16,
    pub min_accent_nontext_contrast_x100: u16,
    pub standard_launcher_enter_ms: u64,
    pub reduced_launcher_enter_ms: u64,
}

impl ThemeSmokeReport {
    pub fn new() -> Self {
        let ctx = egui::Context::default();
        apply_theme(&ctx, ThemeMode::Dark);
        let dark = ctx.style().visuals.clone();
        apply_theme(&ctx, ThemeMode::Light);
        let light = ctx.style().visuals.clone();
        let a11y = AccessibilityContext {
            reduce_motion: false,
            reduce_transparency: false,
            high_contrast: true,
            bold_text: false,
        };
        Self {
            dark_surface_0: dark.panel_fill,
            dark_surface_1: dark.window_fill,
            dark_surface_2: dark.widgets.inactive.bg_fill,
            dark_surface_3: dark.widgets.hovered.bg_fill,
            light_surface_0: light.panel_fill,
            light_surface_1: light.window_fill,
            light_surface_2: light.widgets.inactive.bg_fill,
            light_surface_3: light.widgets.hovered.bg_fill,
            dark_selection: dark.selection.bg_fill,
            light_selection: light.selection.bg_fill,
            dark_accent: Color::accent_base_for(EffectiveTheme::Dark, &a11y),
            light_accent: Color::accent_base_for(EffectiveTheme::Light, &a11y),
            dark_accent_weak_alpha: Color::accent_weak_for(EffectiveTheme::Dark, &a11y).a(),
            light_accent_weak_alpha: Color::accent_weak_for(EffectiveTheme::Light, &a11y).a(),
            dark_mode_applied: dark.dark_mode,
            light_mode_applied: !light.dark_mode,
            high_contrast_focus_ring_width: a11y.focus_ring_width() as u32,
            min_text_contrast_x100: contrast_x100(super::contrast::min_text_contrast_ratio()),
            min_accent_nontext_contrast_x100: contrast_x100(
                super::contrast::min_accent_nontext_contrast_ratio(),
            ),
            standard_launcher_enter_ms: duration_ms(MotionContext::standard().launcher_enter()),
            reduced_launcher_enter_ms: duration_ms(MotionContext::reduced().launcher_enter()),
        }
    }

    pub fn pass(&self) -> bool {
        self.dark_mode_applied
            && self.light_mode_applied
            && self.dark_surface_0 != self.light_surface_0
            && self.dark_surface_1 != self.light_surface_1
            && layered_dark_surfaces(self)
            && layered_light_surfaces(self)
            && self.dark_selection != self.light_selection
            && self.dark_accent != self.light_accent
            && self.dark_accent_weak_alpha == 82
            && self.light_accent_weak_alpha == 56
            && self.dark_surface_0.r() >= 24
            && self.light_surface_0 != egui::Color32::WHITE
            && self.high_contrast_focus_ring_width == 3
            && self.min_text_contrast_x100 >= 450
            && self.min_accent_nontext_contrast_x100 >= 300
            && self.standard_launcher_enter_ms == 320
            && self.reduced_launcher_enter_ms == 0
    }

    pub fn summary(&self, surface: &str) -> String {
        format!(
            "{surface}_theme_smoke {}\ndark_surface_0={}\ndark_surface_1={}\ndark_surface_2={}\ndark_surface_3={}\nlight_surface_0={}\nlight_surface_1={}\nlight_surface_2={}\nlight_surface_3={}\ndark_selection={}\nlight_selection={}\ndark_accent={}\nlight_accent={}\ndark_accent_weak_alpha={}\nlight_accent_weak_alpha={}\ndark_mode_applied={}\nlight_mode_applied={}\nhigh_contrast_focus_ring_width={}\nmin_text_contrast_x100={}\nmin_accent_nontext_contrast_x100={}\nstandard_launcher_enter_ms={}\nreduced_launcher_enter_ms={}",
            if self.pass() { "PASS" } else { "FAIL" },
            color_hex(self.dark_surface_0),
            color_hex(self.dark_surface_1),
            color_hex(self.dark_surface_2),
            color_hex(self.dark_surface_3),
            color_hex(self.light_surface_0),
            color_hex(self.light_surface_1),
            color_hex(self.light_surface_2),
            color_hex(self.light_surface_3),
            color_hex(self.dark_selection),
            color_hex(self.light_selection),
            color_hex(self.dark_accent),
            color_hex(self.light_accent),
            self.dark_accent_weak_alpha,
            self.light_accent_weak_alpha,
            self.dark_mode_applied,
            self.light_mode_applied,
            self.high_contrast_focus_ring_width,
            self.min_text_contrast_x100,
            self.min_accent_nontext_contrast_x100,
            self.standard_launcher_enter_ms,
            self.reduced_launcher_enter_ms
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

fn layered_dark_surfaces(report: &ThemeSmokeReport) -> bool {
    report.dark_surface_0.r() < report.dark_surface_1.r()
        && report.dark_surface_1.r() < report.dark_surface_2.r()
        && report.dark_surface_2.r() < report.dark_surface_3.r()
}

fn layered_light_surfaces(report: &ThemeSmokeReport) -> bool {
    report.light_surface_0.r() > report.light_surface_1.r()
        && report.light_surface_1.r() > report.light_surface_2.r()
        && report.light_surface_2.r() > report.light_surface_3.r()
}

fn duration_ms(duration: Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}

fn contrast_x100(ratio: f32) -> u16 {
    (ratio * 100.0).round().clamp(0.0, f32::from(u16::MAX)) as u16
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
        assert_eq!(color_hex(report.dark_surface_0), "#1C1E22");
        assert_eq!(color_hex(report.light_surface_0), "#FAFBFD");
        assert_eq!(report.high_contrast_focus_ring_width, 3);
        assert_eq!(report.dark_accent_weak_alpha, 82);
        assert_eq!(report.light_accent_weak_alpha, 56);
        assert!(report.min_text_contrast_x100 >= 450);
        assert!(report.min_accent_nontext_contrast_x100 >= 300);
        assert_eq!(report.standard_launcher_enter_ms, 320);
        assert_eq!(report.reduced_launcher_enter_ms, 0);
    }

    #[test]
    fn light_theme_surface_is_tinted_not_pure_white() {
        let report = ThemeSmokeReport::new();

        assert_ne!(report.light_surface_0, egui::Color32::WHITE);
        assert_eq!(color_hex(report.light_surface_0), "#FAFBFD");
    }

    #[test]
    fn theme_profile_reports_runtime_theme_a11y_and_motion_state() {
        let ctx = egui::Context::default();

        let profile = ThemeProfile::apply(&ctx, ThemeMode::Light);

        assert_eq!(profile.requested, ThemeMode::Light);
        assert_eq!(profile.effective, EffectiveTheme::Light);
        assert!(!ctx.style().visuals.dark_mode);
        assert_eq!(profile.focus_ring_width, 2);
        assert!(!profile.high_contrast);
    }
}
