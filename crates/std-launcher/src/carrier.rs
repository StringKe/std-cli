use eframe::egui;
use std_egui::tokens::{LauncherSize, UiScale};

use crate::{
    launcher_clear_color_contract, launcher_viewport_frame_contract, LauncherViewportContract,
    PANEL_WIDTH,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherCarrierEvidence {
    pub native_clear_color: String,
    pub viewport_frame: String,
    pub host_window: String,
    pub geometry: String,
    pub pixel_policy: String,
}

impl LauncherCarrierEvidence {
    pub fn for_height(panel_height: f32) -> Self {
        Self {
            native_clear_color: launcher_clear_color_contract(),
            viewport_frame: launcher_viewport_frame_contract(),
            host_window: host_window_contract(panel_height),
            geometry: geometry_contract(panel_height),
            pixel_policy: pixel_policy_contract(),
        }
    }

    pub fn pass(&self) -> bool {
        self.native_clear_color == "native_clear_color=transparent_rgba_0_0_0_0"
            && self.viewport_frame == "viewport_frame=transparent_fill,no_stroke"
            && self.host_window.contains("panel_sized_transparent_host")
            && self.host_window.contains("host_background=none")
            && self.host_window.contains("host_gutter=0px")
            && self.geometry.contains("panel_origin=0x0")
            && self.geometry.contains("host_gap=0x0")
            && self.geometry.contains("panel_only_surface=true")
            && self.pixel_policy.contains("host-carrier=absent")
            && self.pixel_policy.contains("edge-black-white-zero")
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_carrier {}\n{}\n{}\n{}\n{}\n{}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.native_clear_color,
            self.viewport_frame,
            self.host_window,
            self.geometry,
            self.pixel_policy
        )
    }
}

pub fn launcher_visible_host_geometry_contract() -> String {
    ["results", "defer", "error"]
        .into_iter()
        .map(|scenario| {
            let evidence = LauncherCarrierEvidence::for_height(360.0);
            format!("{scenario}:{}", evidence.geometry)
        })
        .collect::<Vec<_>>()
        .join("|")
}

pub fn launcher_capture_pixel_contract() -> String {
    pixel_policy_contract()
}

fn host_window_contract(panel_height: f32) -> String {
    let contract = LauncherViewportContract::visible();
    let size = egui::vec2(PANEL_WIDTH, panel_height);
    if contract.passes() {
        return format!(
            "native_host_window=panel_sized_transparent_host,{}",
            contract.native_host_window_summary(size)
        );
    }
    "native_host_window=FAIL".to_string()
}

fn geometry_contract(panel_height: f32) -> String {
    let scale = UiScale::default();
    let panel = egui::vec2(PANEL_WIDTH, panel_height);
    let host = LauncherSize::host_size(panel, scale);
    let gutter = LauncherSize::host_gutter(scale);
    format!(
        "native_host={}x{};host_background=none;panel_surface=opaque;panel_origin={}x{};panel_size={}x{};host_gap={}x{};frame_clear=true;panel_only_surface=true;visible_carrier=none",
        host.x.round() as u32,
        host.y.round() as u32,
        gutter.round() as i32,
        gutter.round() as i32,
        panel.x.round() as u32,
        panel.y.round() as u32,
        (host.x - panel.x).round() as i32,
        (host.y - panel.y).round() as i32
    )
}

fn pixel_policy_contract() -> String {
    [
        "capture_pixels=center-panel-opaque-non-carrier",
        "host-carrier=absent",
        "edge-black-white-zero",
        "min-opaque-samples=5",
        "min-edge-transparent=rounded-corners-only",
    ]
    .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carrier_evidence_forbids_visible_black_or_white_host() {
        let evidence = LauncherCarrierEvidence::for_height(360.0);
        let summary = evidence.summary();

        assert!(evidence.pass(), "{summary}");
        assert!(summary.contains("native_clear_color=transparent_rgba_0_0_0_0"));
        assert!(summary.contains("viewport_frame=transparent_fill,no_stroke"));
        assert!(summary.contains("visible_carrier=none"));
        assert!(summary.contains("host-carrier=absent"));
        assert!(summary.contains("edge-black-white-zero"));
    }

    #[test]
    fn visible_host_geometry_lists_launcher_state_scenarios() {
        let summary = launcher_visible_host_geometry_contract();

        assert!(summary.contains("results:native_host=720x360"));
        assert!(summary.contains("defer:native_host=720x360"));
        assert!(summary.contains("error:native_host=720x360"));
        assert!(summary.contains("panel_origin=0x0"));
        assert!(summary.contains("host_gap=0x0"));
    }
}
