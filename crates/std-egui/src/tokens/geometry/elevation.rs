use crate::{
    a11y::AccessibilityContext,
    tokens::{palette, typography::UiScale},
};

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
