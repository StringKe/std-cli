use crate::motion::{MotionBudgetReport, MotionContext, MotionCurve, MotionScene, MotionSpec};
use std::time::Duration;

#[test]
fn standard_motion_uses_documented_launcher_duration() {
    let motion = MotionContext::standard();

    assert_eq!(motion.launcher_enter(), Duration::from_millis(320));
    assert_eq!(motion.launcher_exit(), Duration::from_millis(140));
}

#[test]
fn reduced_motion_collapses_nonessential_durations() {
    let motion = MotionContext::reduced();

    assert_eq!(motion.launcher_enter(), Duration::ZERO);
    assert_eq!(motion.focus_ring(), Duration::ZERO);
    assert_eq!(motion.modal_enter(), Duration::ZERO);
}

#[test]
fn motion_curves_match_docs19_named_tokens() {
    assert_eq!(
        MotionCurve::OutStandard.cubic_bezier(),
        Some([0.2, 0.0, 0.0, 1.0])
    );
    assert_eq!(
        MotionCurve::InStandard.cubic_bezier(),
        Some([0.4, 0.0, 1.0, 1.0])
    );
    assert_eq!(
        MotionCurve::InOut.cubic_bezier(),
        Some([0.4, 0.0, 0.2, 1.0])
    );
    assert_eq!(
        MotionCurve::Snappy.cubic_bezier(),
        Some([0.18, 1.0, 0.22, 1.0])
    );
    assert_eq!(MotionCurve::Linear.cubic_bezier(), None);
}

#[test]
fn scene_specs_cover_docs19_launcher_and_studio_paths() {
    let motion = MotionContext::standard();

    assert_eq!(MotionScene::ALL.len(), 22);
    assert_eq!(
        motion.spec(MotionScene::LauncherEnter),
        MotionSpec {
            scene: MotionScene::LauncherEnter,
            duration: Duration::from_millis(320),
            duration_token: "dur/medium",
            curve: MotionCurve::OutStandard,
            animated_properties: "opacity + y",
            reduced_behavior: "opacity",
        }
    );
    assert_eq!(
        motion.spec(MotionScene::PopoverEnter).duration_token,
        "dur/base"
    );
    assert_eq!(
        motion.spec(MotionScene::SidebarToggle).curve,
        MotionCurve::InOut
    );
    assert_eq!(
        motion.spec(MotionScene::ToastEnter).curve,
        MotionCurve::Snappy
    );
}

#[test]
fn reduced_motion_collapses_non_progress_scene_durations() {
    let motion = MotionContext::reduced();

    for scene in MotionScene::ALL {
        let spec = motion.spec(scene);
        if matches!(
            scene,
            MotionScene::ProgressIndeterminate | MotionScene::ProgressDeterminate
        ) {
            assert_eq!(spec.curve, MotionCurve::Linear);
        } else {
            assert_eq!(spec.duration, Duration::ZERO, "{}", scene.token());
        }
    }
}

#[test]
fn scene_contract_exposes_named_tokens_for_surface_smoke() {
    let contract = MotionContext::standard().scene_contract();

    assert!(contract.contains("launcher-enter:320:ease/out-standard:opacity"));
    assert!(contract.contains("popover-enter:220:ease/out-standard:opacity"));
    assert!(contract.contains("sidebar-toggle:220:ease/in-out:instant"));
    assert!(contract.contains("progress-indeterminate:1200:ease/linear:static"));
}

#[test]
fn motion_budget_reports_p95_frame_time_and_animation_limit() {
    let report = MotionBudgetReport::from_frame_samples("launcher", &[2, 3, 4, 8, 12], 8);

    assert_eq!(report.frame_p95_ms, 12);
    assert!(!report.pass());
    assert!(report.summary().contains("launcher_motion_budget FAIL"));
}

#[test]
fn motion_budget_passes_when_samples_stay_inside_docs19_limits() {
    let report = MotionBudgetReport::from_frame_samples("studio", &[2, 3, 4, 7, 8], 6);

    assert!(report.pass());
    assert!(report.summary().contains("studio_motion_budget PASS"));
    assert!(report.summary().contains("active_animation_limit=8"));
}
