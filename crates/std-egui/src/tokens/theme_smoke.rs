use super::{style::apply_theme, typography::Text, Color, EffectiveTheme, ThemeMode, UiScale};
use crate::{a11y::AccessibilityContext, motion::MotionContext};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub high_contrast_dark_accent_weak_alpha: u8,
    pub high_contrast_light_accent_weak_alpha: u8,
    pub high_contrast_focus_ring_width: u32,
    pub min_text_contrast_x100: u16,
    pub min_secondary_text_contrast_x100: u16,
    pub min_tertiary_text_contrast_x100: u16,
    pub min_high_contrast_secondary_text_contrast_x100: u16,
    pub min_accent_nontext_contrast_x100: u16,
    pub standard_launcher_enter_ms: u64,
    pub reduced_launcher_enter_ms: u64,
    pub typography_contract: String,
    pub status_contract: String,
    pub no_pure_black_white_tokens: bool,
}

impl ThemeSmokeReport {
    pub fn new() -> Self {
        let ctx = egui::Context::default();
        apply_theme(&ctx, ThemeMode::Dark);
        let dark = ctx.style().visuals.clone();
        apply_theme(&ctx, ThemeMode::Light);
        let light = ctx.style().visuals.clone();
        let standard_a11y = a11y(false);
        let high_contrast_a11y = a11y(true);
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
            dark_accent: Color::accent_base_for(EffectiveTheme::Dark, &standard_a11y),
            light_accent: Color::accent_base_for(EffectiveTheme::Light, &standard_a11y),
            dark_accent_weak_alpha: Color::accent_weak_for(EffectiveTheme::Dark, &standard_a11y)
                .a(),
            light_accent_weak_alpha: Color::accent_weak_for(EffectiveTheme::Light, &standard_a11y)
                .a(),
            dark_mode_applied: dark.dark_mode,
            light_mode_applied: !light.dark_mode,
            high_contrast_dark_accent_weak_alpha: Color::accent_weak_for(
                EffectiveTheme::Dark,
                &high_contrast_a11y,
            )
            .a(),
            high_contrast_light_accent_weak_alpha: Color::accent_weak_for(
                EffectiveTheme::Light,
                &high_contrast_a11y,
            )
            .a(),
            high_contrast_focus_ring_width: high_contrast_a11y.focus_ring_width() as u32,
            min_text_contrast_x100: contrast_x100(super::contrast::min_text_contrast_ratio()),
            min_secondary_text_contrast_x100: contrast_x100(
                super::contrast::min_secondary_text_contrast_ratio(),
            ),
            min_tertiary_text_contrast_x100: contrast_x100(
                super::contrast::min_tertiary_text_contrast_ratio(),
            ),
            min_high_contrast_secondary_text_contrast_x100: contrast_x100(
                super::contrast::min_high_contrast_secondary_text_contrast_ratio(),
            ),
            min_accent_nontext_contrast_x100: contrast_x100(
                super::contrast::min_accent_nontext_contrast_ratio(),
            ),
            standard_launcher_enter_ms: duration_ms(MotionContext::standard().launcher_enter()),
            reduced_launcher_enter_ms: duration_ms(MotionContext::reduced().launcher_enter()),
            typography_contract: typography_contract(),
            status_contract: status_contract(&standard_a11y),
            no_pure_black_white_tokens: no_pure_black_white_tokens(&dark, &light),
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
            && self.dark_accent_weak_alpha == 46
            && self.light_accent_weak_alpha == 31
            && self.dark_surface_0.r() >= 24
            && self.light_surface_0 != egui::Color32::WHITE
            && self.high_contrast_dark_accent_weak_alpha == 82
            && self.high_contrast_light_accent_weak_alpha == 56
            && self.high_contrast_focus_ring_width == 3
            && self.min_text_contrast_x100 >= 450
            && self.min_secondary_text_contrast_x100 >= 450
            && self.min_tertiary_text_contrast_x100 >= 300
            && self.min_high_contrast_secondary_text_contrast_x100
                > self.min_secondary_text_contrast_x100
            && self.min_accent_nontext_contrast_x100 >= 300
            && self.standard_launcher_enter_ms == 320
            && self.reduced_launcher_enter_ms == 0
            && self.typography_contract
                == "text=caption:11,footnote:12,body:13,title:15,headline:18,display:24,code:12"
            && self.status_contract == "status=success:#3DCB7C/#138750,warning:#F5B643/#B27500,danger:#FF6A6A/#C8312B,info:#4E9CFF/#0A6BFF"
            && self.no_pure_black_white_tokens
    }

    pub fn summary(&self, surface: &str) -> String {
        format!(
            "{surface}_theme_smoke {}\ndark_surface_0={}\ndark_surface_1={}\ndark_surface_2={}\ndark_surface_3={}\nlight_surface_0={}\nlight_surface_1={}\nlight_surface_2={}\nlight_surface_3={}\ndark_selection={}\nlight_selection={}\ndark_accent={}\nlight_accent={}\ndark_accent_weak_alpha={}\nlight_accent_weak_alpha={}\ndark_mode_applied={}\nlight_mode_applied={}\nhigh_contrast_dark_accent_weak_alpha={}\nhigh_contrast_light_accent_weak_alpha={}\nhigh_contrast_focus_ring_width={}\nmin_text_contrast_x100={}\nmin_secondary_text_contrast_x100={}\nmin_tertiary_text_contrast_x100={}\nmin_high_contrast_secondary_text_contrast_x100={}\nmin_accent_nontext_contrast_x100={}\nstandard_launcher_enter_ms={}\nreduced_launcher_enter_ms={}\ntypography_contract={}\nstatus_contract={}\nno_pure_black_white_tokens={}",
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
            self.high_contrast_dark_accent_weak_alpha,
            self.high_contrast_light_accent_weak_alpha,
            self.high_contrast_focus_ring_width,
            self.min_text_contrast_x100,
            self.min_secondary_text_contrast_x100,
            self.min_tertiary_text_contrast_x100,
            self.min_high_contrast_secondary_text_contrast_x100,
            self.min_accent_nontext_contrast_x100,
            self.standard_launcher_enter_ms,
            self.reduced_launcher_enter_ms,
            self.typography_contract,
            self.status_contract,
            self.no_pure_black_white_tokens
        )
    }
}

impl Default for ThemeSmokeReport {
    fn default() -> Self {
        Self::new()
    }
}

fn a11y(high_contrast: bool) -> AccessibilityContext {
    AccessibilityContext {
        reduce_motion: false,
        reduce_transparency: false,
        high_contrast,
        bold_text: false,
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

fn no_pure_black_white_tokens(dark: &egui::Visuals, light: &egui::Visuals) -> bool {
    [
        dark.panel_fill,
        dark.window_fill,
        dark.widgets.inactive.bg_fill,
        dark.widgets.hovered.bg_fill,
        dark.selection.bg_fill,
        light.panel_fill,
        light.window_fill,
        light.widgets.inactive.bg_fill,
        light.widgets.hovered.bg_fill,
        light.selection.bg_fill,
    ]
    .into_iter()
    .all(|color| color != egui::Color32::BLACK && color != egui::Color32::WHITE)
}

fn typography_contract() -> String {
    let scale = UiScale::default();
    format!(
        "text=caption:{},footnote:{},body:{},title:{},headline:{},display:{},code:{}",
        size_label(Text::caption_for_scale(scale).size),
        size_label(Text::footnote_for_scale(scale).size),
        size_label(Text::body_for_scale(scale).size),
        size_label(Text::title_for_scale(scale).size),
        size_label(Text::headline_for_scale(scale).size),
        size_label(Text::display_for_scale(scale).size),
        size_label(Text::code_for_scale(scale).size)
    )
}

fn status_contract(a11y: &AccessibilityContext) -> String {
    format!(
        "status=success:{}/{},warning:{}/{},danger:{}/{},info:{}/{}",
        color_hex(Color::status_success_for(EffectiveTheme::Dark, a11y)),
        color_hex(Color::status_success_for(EffectiveTheme::Light, a11y)),
        color_hex(Color::status_warning_for(EffectiveTheme::Dark, a11y)),
        color_hex(Color::status_warning_for(EffectiveTheme::Light, a11y)),
        color_hex(Color::status_danger_for(EffectiveTheme::Dark, a11y)),
        color_hex(Color::status_danger_for(EffectiveTheme::Light, a11y)),
        color_hex(Color::accent_base_for(EffectiveTheme::Dark, a11y)),
        color_hex(Color::accent_base_for(EffectiveTheme::Light, a11y))
    )
}

fn size_label(size: f32) -> u32 {
    size.round() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(report.dark_accent_weak_alpha, 46);
        assert_eq!(report.light_accent_weak_alpha, 31);
        assert_eq!(report.high_contrast_dark_accent_weak_alpha, 82);
        assert_eq!(report.high_contrast_light_accent_weak_alpha, 56);
        assert!(report.min_text_contrast_x100 >= 450);
        assert!(report.min_secondary_text_contrast_x100 >= 450);
        assert!(report.min_tertiary_text_contrast_x100 >= 300);
        assert!(
            report.min_high_contrast_secondary_text_contrast_x100
                > report.min_secondary_text_contrast_x100
        );
        assert!(report.min_accent_nontext_contrast_x100 >= 300);
        assert_eq!(report.standard_launcher_enter_ms, 320);
        assert_eq!(report.reduced_launcher_enter_ms, 0);
        assert_eq!(
            report.typography_contract,
            "text=caption:11,footnote:12,body:13,title:15,headline:18,display:24,code:12"
        );
        assert_eq!(
            report.status_contract,
            "status=success:#3DCB7C/#138750,warning:#F5B643/#B27500,danger:#FF6A6A/#C8312B,info:#4E9CFF/#0A6BFF"
        );
        assert!(report.no_pure_black_white_tokens);
    }

    #[test]
    fn light_theme_surface_is_tinted_not_pure_white() {
        let report = ThemeSmokeReport::new();

        assert_ne!(report.light_surface_0, egui::Color32::WHITE);
        assert_eq!(color_hex(report.light_surface_0), "#FAFBFD");
    }
}
