use egui::{Color32, FontData, FontDefinitions, FontFamily, FontId, Stroke, TextStyle};
use std::{path::Path, sync::Arc};

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
        themed(
            ctx,
            Color32::from_rgb(14, 15, 17),
            Color32::from_rgb(255, 255, 255),
        )
    }

    pub fn bg_surface_1(ctx: &egui::Context) -> Color32 {
        themed(
            ctx,
            Color32::from_rgb(22, 24, 27),
            Color32::from_rgb(247, 248, 250),
        )
    }

    pub fn bg_surface_2(ctx: &egui::Context) -> Color32 {
        themed(
            ctx,
            Color32::from_rgb(30, 33, 38),
            Color32::from_rgb(238, 240, 243),
        )
    }

    pub fn bg_surface_3(ctx: &egui::Context) -> Color32 {
        themed(
            ctx,
            Color32::from_rgb(38, 42, 48),
            Color32::from_rgb(227, 230, 234),
        )
    }

    pub fn fg_primary(ctx: &egui::Context) -> Color32 {
        themed(
            ctx,
            Color32::from_rgb(236, 238, 241),
            Color32::from_rgb(26, 28, 32),
        )
    }

    pub fn fg_secondary(ctx: &egui::Context) -> Color32 {
        themed(
            ctx,
            Color32::from_rgb(181, 186, 193),
            Color32::from_rgb(75, 80, 87),
        )
    }

    pub fn fg_tertiary(_ctx: &egui::Context) -> Color32 {
        Color32::from_rgb(122, 128, 137)
    }

    pub fn stroke_divider(ctx: &egui::Context) -> Color32 {
        themed(
            ctx,
            Color32::from_rgb(38, 42, 48),
            Color32::from_rgb(227, 230, 234),
        )
    }

    pub fn stroke_border(ctx: &egui::Context) -> Color32 {
        themed(
            ctx,
            Color32::from_rgb(52, 57, 63),
            Color32::from_rgb(208, 212, 217),
        )
    }

    pub fn accent_base(ctx: &egui::Context) -> Color32 {
        themed(
            ctx,
            Color32::from_rgb(78, 156, 255),
            Color32::from_rgb(10, 107, 255),
        )
    }

    pub fn accent_weak(ctx: &egui::Context) -> Color32 {
        themed(
            ctx,
            Color32::from_rgba_premultiplied(78, 156, 255, 46),
            Color32::from_rgba_premultiplied(10, 107, 255, 31),
        )
    }

    pub fn status_success(ctx: &egui::Context) -> Color32 {
        themed(
            ctx,
            Color32::from_rgb(61, 203, 124),
            Color32::from_rgb(19, 135, 80),
        )
    }

    pub fn status_warning(ctx: &egui::Context) -> Color32 {
        themed(
            ctx,
            Color32::from_rgb(245, 182, 67),
            Color32::from_rgb(178, 117, 0),
        )
    }

    pub fn status_danger(ctx: &egui::Context) -> Color32 {
        themed(
            ctx,
            Color32::from_rgb(255, 106, 106),
            Color32::from_rgb(200, 49, 43),
        )
    }
}

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

    pub fn code() -> FontId {
        FontId::new(12.0, FontFamily::Monospace)
    }
}

pub fn apply_theme(ctx: &egui::Context, mode: ThemeMode) {
    install_fonts(ctx);
    let effective = effective_theme(ctx, mode);
    let mut visuals = match effective {
        EffectiveTheme::Dark => egui::Visuals::dark(),
        EffectiveTheme::Light => egui::Visuals::light(),
    };
    visuals.panel_fill = Color::bg_surface_0(ctx);
    visuals.window_fill = Color::bg_surface_1(ctx);
    visuals.extreme_bg_color = Color::bg_surface_0(ctx);
    visuals.faint_bg_color = Color::bg_surface_1(ctx);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, Color::fg_primary(ctx));
    visuals.widgets.inactive.bg_fill = Color::bg_surface_2(ctx);
    visuals.widgets.hovered.bg_fill = Color::bg_surface_3(ctx);
    visuals.widgets.active.bg_fill = Color::accent_weak(ctx);
    visuals.selection.bg_fill = Color::accent_weak(ctx);
    visuals.selection.stroke = Stroke::new(1.0, Color::accent_base(ctx));
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
    ctx.input(|input| {
        input.events.iter().any(|event| {
            matches!(
                event,
                egui::Event::Ime(egui::ImeEvent::Enabled)
                    | egui::Event::Ime(egui::ImeEvent::Preedit(_))
            )
        })
    })
}

pub fn reduce_motion() -> bool {
    std::env::var("STD_REDUCE_MOTION")
        .or_else(|_| std::env::var("STDCLI_REDUCE_MOTION"))
        .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "on"))
        .unwrap_or(false)
}

fn themed(ctx: &egui::Context, dark: Color32, light: Color32) -> Color32 {
    match effective_theme(ctx, ThemeMode::System) {
        EffectiveTheme::Dark => dark,
        EffectiveTheme::Light => light,
    }
}

fn effective_theme(ctx: &egui::Context, mode: ThemeMode) -> EffectiveTheme {
    match mode {
        ThemeMode::Dark => EffectiveTheme::Dark,
        ThemeMode::Light => EffectiveTheme::Light,
        ThemeMode::System if ctx.style().visuals.dark_mode => EffectiveTheme::Dark,
        ThemeMode::System => EffectiveTheme::Light,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_mode_resolves_config_values() {
        assert_eq!(ThemeMode::resolve("dark"), ThemeMode::Dark);
        assert_eq!(ThemeMode::resolve("light"), ThemeMode::Light);
        assert_eq!(ThemeMode::resolve("system"), ThemeMode::System);
        assert_eq!(ThemeMode::resolve("unknown"), ThemeMode::System);
    }

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
    fn apply_theme_sets_expected_visual_mode() {
        let ctx = egui::Context::default();
        apply_theme(&ctx, ThemeMode::Light);
        assert!(!ctx.style().visuals.dark_mode);
        apply_theme(&ctx, ThemeMode::Dark);
        assert!(ctx.style().visuals.dark_mode);
    }
}
