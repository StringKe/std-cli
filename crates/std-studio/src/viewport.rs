use eframe::egui;
use std_studio::{HostWindowPolicy, StudioWorkspacePolicy};

pub(crate) const STUDIO_WINDOW_SIZE: [f32; 2] = [1280.0, 800.0];
pub(crate) const STUDIO_MIN_WINDOW_SIZE: [f32; 2] = [1080.0, 640.0];

pub(crate) fn studio_native_options() -> eframe::NativeOptions {
    let policy = StudioWorkspacePolicy::studio_v1();
    debug_assert_eq!(
        policy.host_window,
        HostWindowPolicy::SingleBorderlessEguiViewport
    );
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(STUDIO_WINDOW_SIZE)
            .with_min_inner_size(STUDIO_MIN_WINDOW_SIZE)
            .with_resizable(true)
            .with_decorations(false),
        ..Default::default()
    }
}

pub(crate) fn studio_host_viewport_contract() -> &'static str {
    "host_viewport=single-borderless-egui,decorations=false,resizable=true,native-child-windows=false"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn studio_host_viewport_matches_single_egui_window_spec() {
        let options = studio_native_options();
        let description = format!("{:?}", options.viewport);

        assert!(description.contains("inner_size: Some([1280.0 800.0])"));
        assert!(description.contains("min_inner_size: Some([1080.0 640.0])"));
        assert!(description.contains("resizable: Some(true)"));
        assert!(description.contains("decorations: Some(false)"));
        assert!(!StudioWorkspacePolicy::studio_v1().allows_native_child_windows());
        assert_eq!(
            studio_host_viewport_contract(),
            "host_viewport=single-borderless-egui,decorations=false,resizable=true,native-child-windows=false"
        );
    }
}
