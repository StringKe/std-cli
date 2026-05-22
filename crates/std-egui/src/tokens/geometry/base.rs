use crate::tokens::typography::UiScale;

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
