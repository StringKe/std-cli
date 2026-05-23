#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LauncherPreviewScenario {
    pub(crate) theme: &'static str,
    pub(crate) state: &'static str,
}

impl LauncherPreviewScenario {
    pub(crate) fn label(&self) -> String {
        format!("{}-{}", self.theme, self.state)
    }

    pub(crate) fn command(&self) -> String {
        format!(
            "STD_ALLOW_UI_PREVIEW=1 target/ui-capture/debug/std-launcher --ui-preview {} {} 8000",
            self.theme, self.state
        )
    }
}

pub(crate) fn preview_window_title() -> &'static str {
    "std-cli-Launcher"
}

pub(crate) fn preview_capture_contract() -> &'static str {
    "panel-sized-transparent-host,opaque-panel-surface,opt-in-only,checkout-binary-only,blocked-in-STD_TEST_MODE,no-default-window,host-gutter-0px,no-host-background,no-shadow-clip"
}

pub(crate) fn preview_matrix() -> Vec<LauncherPreviewScenario> {
    [
        LauncherPreviewScenario {
            theme: "light",
            state: "collapsed",
        },
        LauncherPreviewScenario {
            theme: "dark",
            state: "collapsed",
        },
        LauncherPreviewScenario {
            theme: "light",
            state: "empty",
        },
        LauncherPreviewScenario {
            theme: "dark",
            state: "empty",
        },
        LauncherPreviewScenario {
            theme: "light",
            state: "results",
        },
        LauncherPreviewScenario {
            theme: "dark",
            state: "results",
        },
        LauncherPreviewScenario {
            theme: "light",
            state: "no-results",
        },
        LauncherPreviewScenario {
            theme: "dark",
            state: "no-results",
        },
        LauncherPreviewScenario {
            theme: "light",
            state: "searching",
        },
        LauncherPreviewScenario {
            theme: "dark",
            state: "searching",
        },
        LauncherPreviewScenario {
            theme: "light",
            state: "loading",
        },
        LauncherPreviewScenario {
            theme: "dark",
            state: "loading",
        },
        LauncherPreviewScenario {
            theme: "light",
            state: "executing",
        },
        LauncherPreviewScenario {
            theme: "dark",
            state: "executing",
        },
        LauncherPreviewScenario {
            theme: "light",
            state: "defer",
        },
        LauncherPreviewScenario {
            theme: "dark",
            state: "defer",
        },
        LauncherPreviewScenario {
            theme: "light",
            state: "error",
        },
        LauncherPreviewScenario {
            theme: "dark",
            state: "error",
        },
        LauncherPreviewScenario {
            theme: "light",
            state: "ime",
        },
        LauncherPreviewScenario {
            theme: "dark",
            state: "ime",
        },
        LauncherPreviewScenario {
            theme: "light",
            state: "action-panel",
        },
        LauncherPreviewScenario {
            theme: "dark",
            state: "action-panel",
        },
    ]
    .into_iter()
    .collect()
}

pub(crate) fn required_capture_states(scenarios: &[LauncherPreviewScenario]) -> Vec<String> {
    required_capture_state_labels()
        .iter()
        .filter(|required| {
            scenarios
                .iter()
                .any(|scenario| scenario.label() == **required)
        })
        .map(|state| (*state).to_string())
        .collect()
}

pub(crate) fn required_capture_states_pass(states: &[String]) -> bool {
    states
        == required_capture_state_labels()
            .iter()
            .map(|state| (*state).to_string())
            .collect::<Vec<_>>()
}

pub(crate) fn required_capture_states_summary() -> String {
    format!(
        "required_capture_states={}",
        required_capture_state_labels().join(",")
    )
}

pub(crate) fn required_capture_state_labels() -> &'static [&'static str] {
    &[
        "light-collapsed",
        "dark-collapsed",
        "light-empty",
        "dark-empty",
        "light-results",
        "dark-results",
        "light-no-results",
        "dark-no-results",
        "light-searching",
        "dark-searching",
        "light-loading",
        "dark-loading",
        "light-executing",
        "dark-executing",
        "light-defer",
        "dark-defer",
        "light-error",
        "dark-error",
        "light-ime",
        "dark-ime",
        "light-action-panel",
        "dark-action-panel",
    ]
}
