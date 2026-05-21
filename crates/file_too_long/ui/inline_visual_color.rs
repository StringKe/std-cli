mod egui {
    pub struct Color32;

    impl Color32 {
        pub fn from_rgb(_: u8, _: u8, _: u8) -> Self {
            Self
        }
    }
}

fn main() {
    let _color = egui::Color32::from_rgb(28, 30, 34);
}
