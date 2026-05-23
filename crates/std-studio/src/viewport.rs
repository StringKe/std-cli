use eframe::egui;
use std_studio::StudioWorkspacePolicy;

pub(crate) const STUDIO_WINDOW_SIZE: [f32; 2] = [1280.0, 800.0];
pub(crate) const STUDIO_MIN_WINDOW_SIZE: [f32; 2] = [1080.0, 640.0];

pub(crate) fn studio_native_options() -> eframe::NativeOptions {
    let policy = StudioWorkspacePolicy::studio_v1();
    let contract = policy.host_viewport_contract();
    debug_assert!(contract.passes());
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([contract.width as f32, contract.height as f32])
            .with_min_inner_size([contract.min_width as f32, contract.min_height as f32])
            .with_resizable(contract.resizable)
            .with_decorations(contract.decorations),
        ..Default::default()
    }
}

pub(crate) fn studio_host_viewport_contract() -> String {
    StudioWorkspacePolicy::studio_v1()
        .host_viewport_contract()
        .summary()
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
            "host_viewport=single-borderless-egui-viewport,panes=internal-egui-workspace-panes,size=1280x800,min=1080x640,decorations=false,resizable=true,native_child_windows=false,detached_panels=false,extra_viewports=false"
        );
    }
}
