use super::{color::effective_theme, Color, EffectiveTheme, ThemeMode};
use crate::a11y::AccessibilityContext;
use egui::{Color32, FontData, FontDefinitions, FontFamily, FontId, Stroke, TextStyle};
use std::{path::Path, sync::Arc};

pub struct Space;

impl Space {
    pub const TWO_XS: i8 = 4;
    pub const XS: i8 = 8;
    pub const SM: i8 = 12;
    pub const MD: i8 = 16;
    pub const LG: i8 = 24;
    pub const XL: i8 = 32;
    pub const TWO_XL: i8 = 48;
}

pub struct Radius;

impl Radius {
    pub const SM: u8 = 4;
    pub const MD: u8 = 8;
    pub const LG: u8 = 12;
    pub const XL: u8 = 16;
}

pub struct Elevation;

impl Elevation {
    pub fn level_2(ctx: &egui::Context) -> egui::Shadow {
        shadow(
            [0, 8],
            24,
            Color32::from_black_alpha(if ctx.style().visuals.dark_mode {
                128
            } else {
                26
            }),
        )
    }

    pub fn level_3(ctx: &egui::Context) -> egui::Shadow {
        shadow(
            [0, 16],
            48,
            Color32::from_black_alpha(if ctx.style().visuals.dark_mode {
                153
            } else {
                41
            }),
        )
    }
}

pub struct Text;

impl Text {
    pub fn caption() -> FontId {
        FontId::new(11.0, FontFamily::Proportional)
    }

    pub fn footnote() -> FontId {
        FontId::new(12.0, FontFamily::Proportional)
    }

    pub fn body() -> FontId {
        FontId::new(13.0, FontFamily::Proportional)
    }

    pub fn title() -> FontId {
        FontId::new(15.0, FontFamily::Proportional)
    }

    pub fn headline() -> FontId {
        FontId::new(18.0, FontFamily::Proportional)
    }

    pub fn display() -> FontId {
        FontId::new(24.0, FontFamily::Proportional)
    }

    pub fn code() -> FontId {
        FontId::new(12.0, FontFamily::Monospace)
    }
}

pub fn apply_theme(ctx: &egui::Context, mode: ThemeMode) {
    install_fonts(ctx);
    let effective = effective_theme(ctx, mode);
    let a11y = AccessibilityContext::from_env();
    let mut visuals = match effective {
        EffectiveTheme::Dark => egui::Visuals::dark(),
        EffectiveTheme::Light => egui::Visuals::light(),
    };
    visuals.panel_fill = Color::bg_surface_0_for(effective, &a11y);
    visuals.window_fill = Color::bg_surface_1_for(effective, &a11y);
    visuals.extreme_bg_color = Color::bg_surface_0_for(effective, &a11y);
    visuals.faint_bg_color = Color::bg_surface_1_for(effective, &a11y);
    visuals.widgets.noninteractive.fg_stroke =
        Stroke::new(1.0, Color::fg_primary_for(effective, &a11y));
    visuals.widgets.inactive.bg_fill = Color::bg_surface_2_for(effective, &a11y);
    visuals.widgets.hovered.bg_fill = Color::bg_surface_3_for(effective, &a11y);
    visuals.widgets.active.bg_fill = Color::accent_weak_for(effective, &a11y);
    visuals.selection.bg_fill = Color::accent_weak_for(effective, &a11y);
    visuals.selection.stroke = Stroke::new(1.0, Color::accent_base_for(effective, &a11y));
    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(TextStyle::Small, Text::caption());
    style.text_styles.insert(TextStyle::Body, Text::body());
    style.text_styles.insert(TextStyle::Button, Text::body());
    style
        .text_styles
        .insert(TextStyle::Heading, Text::headline());
    style.text_styles.insert(TextStyle::Monospace, Text::code());
    ctx.set_style(style);
}

pub fn install_fonts(ctx: &egui::Context) {
    let fonts_installed = ctx.data(|data| {
        data.get_temp::<bool>(egui::Id::new("std.egui.fonts.installed"))
            .unwrap_or(false)
    });
    if fonts_installed {
        return;
    }
    let Some(font_data) = load_cjk_font() else {
        return;
    };
    let mut fonts = FontDefinitions::default();
    fonts
        .font_data
        .insert("std-cjk".to_string(), Arc::new(font_data));
    for family in [FontFamily::Proportional, FontFamily::Monospace] {
        fonts
            .families
            .get_mut(&family)
            .expect("default egui font family exists")
            .push("std-cjk".to_string());
    }
    ctx.set_fonts(fonts);
    ctx.data_mut(|data| data.insert_temp(egui::Id::new("std.egui.fonts.installed"), true));
}

fn load_cjk_font() -> Option<FontData> {
    ["/System/Library/Fonts/Supplemental/Arial Unicode.ttf"]
        .iter()
        .find_map(|path| std::fs::read(Path::new(path)).ok())
        .map(FontData::from_owned)
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

fn shadow(offset: [i8; 2], blur: u8, color: Color32) -> egui::Shadow {
    egui::Shadow {
        offset,
        blur,
        spread: 0,
        color,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exported_spacing_matches_eight_point_grid() {
        assert_eq!(Space::TWO_XS, 4);
        assert_eq!(Space::XS, 8);
        assert_eq!(Space::SM, 12);
        assert_eq!(Space::MD, 16);
        assert_eq!(Space::LG, 24);
        assert_eq!(Space::XL, 32);
        assert_eq!(Space::TWO_XL, 48);
    }

    #[test]
    fn exported_elevation_matches_documented_shadow_levels() {
        let ctx = egui::Context::default();
        apply_theme(&ctx, ThemeMode::Dark);
        let level_2 = Elevation::level_2(&ctx);
        let level_3 = Elevation::level_3(&ctx);

        assert_eq!(level_2.offset, [0, 8]);
        assert_eq!(level_2.blur, 24);
        assert_eq!(level_3.offset, [0, 16]);
        assert_eq!(level_3.blur, 48);

        apply_theme(&ctx, ThemeMode::Light);
        assert!(Elevation::level_3(&ctx).color.a() < level_3.color.a());
    }

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
        assert_eq!(visuals.panel_fill, Color32::from_rgb(255, 255, 255));
        assert_eq!(visuals.window_fill, Color32::from_rgb(247, 248, 250));
        assert_eq!(
            visuals.selection.bg_fill,
            Color32::from_rgba_premultiplied(10, 107, 255, 31)
        );
    }
}
