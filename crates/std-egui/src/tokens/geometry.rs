use super::{palette, typography::UiScale};
use crate::a11y::AccessibilityContext;

pub struct Space;

impl Space {
    pub const TWO_XS: i8 = 4;
    pub const XS: i8 = 8;
    pub const SM: i8 = 12;
    pub const MD: i8 = 16;
    pub const LG: i8 = 24;
    pub const XL: i8 = 32;
    pub const TWO_XL: i8 = 48;

    pub fn two_xs() -> i8 {
        UiScale::from_env().i8(Self::TWO_XS)
    }

    pub fn xs() -> i8 {
        UiScale::from_env().i8(Self::XS)
    }

    pub fn sm() -> i8 {
        UiScale::from_env().i8(Self::SM)
    }

    pub fn md() -> i8 {
        UiScale::from_env().i8(Self::MD)
    }

    pub fn lg() -> i8 {
        UiScale::from_env().i8(Self::LG)
    }

    pub fn xl() -> i8 {
        UiScale::from_env().i8(Self::XL)
    }

    pub fn two_xl() -> i8 {
        UiScale::from_env().i8(Self::TWO_XL)
    }

    pub(crate) fn md_for_scale(scale: UiScale) -> f32 {
        scale.f32(Self::MD as f32)
    }
}

pub struct Radius;

impl Radius {
    pub const SM: u8 = 4;
    pub const MD: u8 = 8;
    pub const LG: u8 = 12;
    pub const XL: u8 = 16;

    pub fn sm() -> u8 {
        UiScale::from_env().u8(Self::SM)
    }

    pub fn md() -> u8 {
        UiScale::from_env().u8(Self::MD)
    }

    pub fn lg() -> u8 {
        UiScale::from_env().u8(Self::LG)
    }

    pub fn xl() -> u8 {
        UiScale::from_env().u8(Self::XL)
    }
}

pub struct Elevation;

impl Elevation {
    pub fn level_2(ctx: &egui::Context) -> egui::Shadow {
        let a11y = AccessibilityContext::from_env();
        Self::level_2_for_scale(ctx, UiScale::from_env(), &a11y)
    }

    pub(crate) fn level_2_for_scale(
        ctx: &egui::Context,
        scale: UiScale,
        a11y: &AccessibilityContext,
    ) -> egui::Shadow {
        scaled_shadow(
            [0, scale.i8(8)],
            elevation_blur(scale.u8(24), a11y),
            palette::shadow_alpha(if ctx.style().visuals.dark_mode {
                128
            } else {
                26
            }),
        )
    }

    pub fn level_3(ctx: &egui::Context) -> egui::Shadow {
        let a11y = AccessibilityContext::from_env();
        Self::level_3_for_scale(ctx, UiScale::from_env(), &a11y)
    }

    pub(crate) fn level_3_for_scale(
        ctx: &egui::Context,
        scale: UiScale,
        a11y: &AccessibilityContext,
    ) -> egui::Shadow {
        scaled_shadow(
            [0, scale.i8(16)],
            elevation_blur(scale.u8(48), a11y),
            palette::shadow_alpha(if ctx.style().visuals.dark_mode {
                153
            } else {
                41
            }),
        )
    }
}

pub(crate) fn elevation_blur(default_blur: u8, a11y: &AccessibilityContext) -> u8 {
    if a11y.reduce_transparency {
        4
    } else {
        default_blur
    }
}

fn scaled_shadow(offset: [i8; 2], blur: u8, color: egui::Color32) -> egui::Shadow {
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
    fn scaled_spacing_and_radius_follow_ui_zoom() {
        let scale = UiScale::new(1.5);

        assert_eq!(scale.i8(Space::SM), 18);
        assert_eq!(scale.u8(Radius::XL), 24);
        assert_eq!(Space::md_for_scale(scale), 24.0);
    }

    #[test]
    fn exported_elevation_matches_documented_shadow_levels() {
        let ctx = egui::Context::default();
        let a11y = AccessibilityContext {
            reduce_motion: false,
            reduce_transparency: false,
            high_contrast: false,
            bold_text: false,
        };
        let level_2 = Elevation::level_2_for_scale(&ctx, UiScale::default(), &a11y);
        let level_3 = Elevation::level_3_for_scale(&ctx, UiScale::default(), &a11y);

        assert_eq!(level_2.offset, [0, 8]);
        assert_eq!(level_2.blur, 24);
        assert_eq!(level_3.offset, [0, 16]);
        assert_eq!(level_3.blur, 48);
    }

    #[test]
    fn reduce_transparency_uses_harder_elevation_edges() {
        let a11y = AccessibilityContext {
            reduce_motion: false,
            reduce_transparency: true,
            high_contrast: false,
            bold_text: false,
        };

        assert_eq!(elevation_blur(24, &a11y), 4);
        assert_eq!(elevation_blur(48, &a11y), 4);
    }

    #[test]
    fn elevation_shadow_geometry_scales_with_ui_zoom() {
        let ctx = egui::Context::default();
        let a11y = AccessibilityContext {
            reduce_motion: false,
            reduce_transparency: false,
            high_contrast: false,
            bold_text: false,
        };

        let level_3 = Elevation::level_3_for_scale(&ctx, UiScale::new(1.5), &a11y);

        assert_eq!(level_3.offset, [0, 24]);
        assert_eq!(level_3.blur, 72);
    }
}
