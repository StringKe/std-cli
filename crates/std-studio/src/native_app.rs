use crate::{viewport::studio_native_options, StudioEguiApp};

pub(crate) fn run_studio_native_app() -> eframe::Result<()> {
    run_studio_native_app_with(StudioEguiApp::default())
}

pub(crate) fn run_studio_native_app_with(app: StudioEguiApp) -> eframe::Result<()> {
    eframe::run_native(
        "std-cli Studio",
        studio_native_options(),
        Box::new(|_cc| Ok(Box::new(app))),
    )
}

pub(crate) fn native_app_blocked_by_test_mode() -> Option<&'static str> {
    std_core::std_test_mode_enabled()
        .then_some("studio_native_app SKIP reason=STD_TEST_MODE blocked native app startup")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_blocks_native_studio_startup() {
        assert_eq!(
            native_app_blocked_by_test_mode(),
            Some("studio_native_app SKIP reason=STD_TEST_MODE blocked native app startup")
        );
    }
}
