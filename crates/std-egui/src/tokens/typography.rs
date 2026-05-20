use crate::a11y::AccessibilityContext;
use egui::{FontData, FontDefinitions, FontFamily, FontId, FontTweak};
use std::{path::Path, sync::Arc};

const MIN_UI_SCALE: f32 = 0.85;
const MAX_UI_SCALE: f32 = 1.5;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UiScale {
    value: f32,
}

impl UiScale {
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(MIN_UI_SCALE, MAX_UI_SCALE),
        }
    }

    pub fn from_env() -> Self {
        std::env::var("STD_UI_ZOOM")
            .ok()
            .and_then(|value| value.parse::<f32>().ok())
            .map(Self::new)
            .unwrap_or_default()
    }

    pub fn value(self) -> f32 {
        self.value
    }

    fn font(self, size: f32, family: FontFamily) -> FontId {
        FontId::new(size * self.value, family)
    }
}

impl Default for UiScale {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

pub struct Text;

impl Text {
    pub fn caption() -> FontId {
        Self::caption_for_scale(UiScale::from_env())
    }

    pub fn footnote() -> FontId {
        Self::footnote_for_scale(UiScale::from_env())
    }

    pub fn body() -> FontId {
        Self::body_for_scale(UiScale::from_env())
    }

    pub fn title() -> FontId {
        Self::title_for_scale(UiScale::from_env())
    }

    pub fn headline() -> FontId {
        Self::headline_for_scale(UiScale::from_env())
    }

    pub fn display() -> FontId {
        Self::display_for_scale(UiScale::from_env())
    }

    pub fn code() -> FontId {
        Self::code_for_scale(UiScale::from_env())
    }

    pub(crate) fn caption_for_scale(scale: UiScale) -> FontId {
        scale.font(11.0, FontFamily::Proportional)
    }

    pub(crate) fn footnote_for_scale(scale: UiScale) -> FontId {
        scale.font(12.0, FontFamily::Proportional)
    }

    pub(crate) fn body_for_scale(scale: UiScale) -> FontId {
        scale.font(13.0, FontFamily::Proportional)
    }

    pub(crate) fn title_for_scale(scale: UiScale) -> FontId {
        scale.font(15.0, FontFamily::Proportional)
    }

    pub(crate) fn headline_for_scale(scale: UiScale) -> FontId {
        scale.font(18.0, FontFamily::Proportional)
    }

    pub(crate) fn display_for_scale(scale: UiScale) -> FontId {
        scale.font(24.0, FontFamily::Proportional)
    }

    pub(crate) fn code_for_scale(scale: UiScale) -> FontId {
        scale.font(12.0, FontFamily::Monospace)
    }
}

pub fn install_fonts(ctx: &egui::Context) {
    let a11y = AccessibilityContext::from_env();
    install_fonts_for_a11y(ctx, &a11y);
}

pub(crate) fn install_fonts_for_a11y(ctx: &egui::Context, a11y: &AccessibilityContext) {
    let profile = font_profile(a11y);
    let fonts_installed = ctx.data(|data| {
        data.get_temp::<String>(egui::Id::new("std.egui.fonts.profile"))
            .is_some_and(|installed| installed == profile)
    });
    if fonts_installed {
        return;
    }
    let Some(font_data) = load_cjk_font() else {
        return;
    };
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "std-cjk".to_string(),
        Arc::new(font_data.tweak(font_tweak(a11y))),
    );
    for family in [FontFamily::Proportional, FontFamily::Monospace] {
        fonts
            .families
            .get_mut(&family)
            .expect("default egui font family exists")
            .push("std-cjk".to_string());
    }
    ctx.set_fonts(fonts);
    ctx.data_mut(|data| data.insert_temp(egui::Id::new("std.egui.fonts.profile"), profile));
}

pub(crate) fn font_profile(a11y: &AccessibilityContext) -> String {
    if a11y.bold_text {
        "bold-text".to_string()
    } else {
        "standard".to_string()
    }
}

pub(crate) fn font_tweak(a11y: &AccessibilityContext) -> FontTweak {
    FontTweak {
        scale: if a11y.bold_text { 1.015 } else { 1.0 },
        y_offset_factor: 0.0,
        y_offset: 0.0,
    }
}

fn load_cjk_font() -> Option<FontData> {
    ["/System/Library/Fonts/Supplemental/Arial Unicode.ttf"]
        .iter()
        .find_map(|path| std::fs::read(Path::new(path)).ok())
        .map(FontData::from_owned)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    #[test]
    fn ui_scale_clamps_to_accessibility_zoom_range() {
        assert_eq!(UiScale::new(0.5).value(), 0.85);
        assert_eq!(UiScale::new(2.0).value(), 1.5);
        assert_eq!(UiScale::new(1.15).value(), 1.15);
    }

    #[test]
    fn text_tokens_scale_from_documented_base_sizes() {
        let scale = UiScale::new(1.5);

        assert_eq!(Text::body_for_scale(scale).size, 19.5);
        assert_eq!(Text::headline_for_scale(scale).size, 27.0);
        assert_eq!(Text::code_for_scale(scale).family, FontFamily::Monospace);
    }

    #[test]
    fn default_scale_preserves_existing_sizes() {
        let scale = UiScale::default();

        assert_eq!(Text::caption_for_scale(scale).size, 11.0);
        assert_eq!(Text::body_for_scale(scale).size, 13.0);
        assert_eq!(Text::display_for_scale(scale).size, 24.0);
    }

    #[test]
    fn ui_scale_reads_environment_zoom() {
        let _guard = env_lock();
        std::env::set_var("STD_UI_ZOOM", "1.25");

        assert_eq!(UiScale::from_env().value(), 1.25);

        std::env::remove_var("STD_UI_ZOOM");
    }

    #[test]
    fn bold_text_uses_distinct_font_profile_without_layout_scale_jump() {
        let standard = AccessibilityContext {
            reduce_motion: false,
            reduce_transparency: false,
            high_contrast: false,
            bold_text: false,
        };
        let bold = AccessibilityContext {
            bold_text: true,
            ..standard.clone()
        };

        assert_eq!(font_profile(&standard), "standard");
        assert_eq!(font_profile(&bold), "bold-text");
        assert_eq!(Text::body_for_scale(UiScale::default()), Text::body());
        assert!(font_tweak(&bold).scale > font_tweak(&standard).scale);
    }
}
