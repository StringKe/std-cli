use crate::semantics_executing::executing_semantics;
use crate::{keyboard::LauncherKey, LauncherState};
use std_egui::{
    a11y::AccessibilityContext,
    i18n::{self, Locale},
    input,
    motion::MotionContext,
    LauncherFeedback,
};
use std_types::{ActionExecution, ActionExecutionStatus, ActionId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherUiSemanticsReport {
    pub search_focused: bool,
    pub result_count: usize,
    pub result_phase: String,
    pub result_mode: String,
    pub empty_phase: String,
    pub empty_mode: String,
    pub empty_result_count: usize,
    pub empty_title: String,
    pub empty_detail: String,
    pub search_reader_label: String,
    pub result_group_label: String,
    pub selected_label: String,
    pub selected_reader_label: String,
    pub selected_position: String,
    pub selected_keycap: String,
    pub selected_action_hint: String,
    pub action_bar_hint: String,
    pub action_panel_actions: String,
    pub action_panel_reader_label: String,
    pub action_panel_open_studio_command: String,
    pub no_results_label: String,
    pub no_results_detail: String,
    pub no_results_fallback: String,
    pub no_results_phase: String,
    pub no_results_enter_query: String,
    pub no_results_ime_enter_blocked: bool,
    pub loading_label: String,
    pub loading_progress: String,
    pub loading_spinner_after_ms: u128,
    pub executing_search_text: String,
    pub running_reader_label: String,
    pub executing_input_enabled: bool,
    pub executing_cancel_shortcut: String,
    pub executing_background_shortcut: String,
    pub defer_feedback_label: String,
    pub defer_actions: String,
    pub failed_feedback_label: String,
    pub completion_reader_label: String,
    pub error_actions: String,
    pub feedback_keyboard_path: String,
    pub error_open_studio_target: String,
    pub error_open_studio_command: String,
    pub shortcut_help_summary: String,
    pub docs23_contract: String,
    pub locale_contract: String,
    pub reduce_motion: bool,
    pub launcher_enter_ms: u128,
    pub focus_ring_width: u32,
}

impl LauncherState {
    pub fn ui_semantics_smoke(query: &str) -> LauncherUiSemanticsReport {
        let mut state = Self::new();
        state.controller.show();
        state.update_query(query);
        let empty = empty_query_semantics();
        let result = result_semantics(&state);
        let no_results = no_result_semantics();
        let loading = loading_semantics();
        let executing = executing_semantics(query);
        let feedback = feedback_semantics();
        let action_panel = action_panel_semantics(query);

        let motion = MotionContext::from_env();
        let a11y = AccessibilityContext::from_env();
        LauncherUiSemanticsReport {
            search_focused: state.controller.focused,
            result_count: state.view.results.len(),
            result_phase: format!("{:?}", state.view.phase),
            result_mode: format!("{:?}", state.view.result_mode),
            empty_phase: empty.phase,
            empty_mode: empty.mode,
            empty_result_count: empty.result_count,
            empty_title: empty.title,
            empty_detail: empty.detail,
            search_reader_label: result.search_reader_label,
            result_group_label: result.result_group_label,
            selected_label: result.selected_label,
            selected_reader_label: result.selected_reader_label,
            selected_position: result.selected_position,
            selected_keycap: result.selected_keycap,
            selected_action_hint: result.selected_action_hint,
            action_bar_hint: result.action_bar_hint,
            action_panel_actions: action_panel.actions,
            action_panel_reader_label: action_panel.reader_label,
            action_panel_open_studio_command: action_panel.open_studio_command,
            no_results_label: no_results.label,
            no_results_detail: no_results.detail,
            no_results_fallback: no_results.fallback,
            no_results_phase: no_results.phase,
            no_results_enter_query: no_results.enter_query,
            no_results_ime_enter_blocked: no_results.ime_enter_blocked,
            loading_label: loading.label,
            loading_progress: loading.progress,
            loading_spinner_after_ms: 200,
            executing_search_text: executing.search_text,
            running_reader_label: feedback.running_label,
            executing_input_enabled: executing.input_enabled,
            executing_cancel_shortcut: executing.cancel_shortcut,
            executing_background_shortcut: executing.background_shortcut,
            defer_feedback_label: feedback.defer_label,
            defer_actions: "Copy,Retry".to_string(),
            failed_feedback_label: feedback.failed_label,
            completion_reader_label: feedback.completion_label,
            error_actions: "Copy,Retry,Open Studio".to_string(),
            feedback_keyboard_path: feedback.keyboard_path,
            error_open_studio_target: feedback.open_studio_target,
            error_open_studio_command: feedback.open_studio_command,
            shortcut_help_summary: shortcut_help_semantics(),
            docs23_contract: "docs/23#launcher-screen-reader".to_string(),
            locale_contract: format!("{:?},{:?}", Locale::ZhCn, Locale::EnUs)
                .replace("ZhCn", "zh-CN")
                .replace("EnUs", "en-US"),
            reduce_motion: motion.is_reduced(),
            launcher_enter_ms: motion.launcher_enter().as_millis(),
            focus_ring_width: a11y.focus_ring_width() as u32,
        }
    }
}

struct ResultSemantics {
    search_reader_label: String,
    result_group_label: String,
    selected_label: String,
    selected_reader_label: String,
    selected_position: String,
    selected_keycap: String,
    selected_action_hint: String,
    action_bar_hint: String,
}

struct EmptyQuerySemantics {
    phase: String,
    mode: String,
    result_count: usize,
    title: String,
    detail: String,
}

struct NoResultSemantics {
    label: String,
    detail: String,
    fallback: String,
    phase: String,
    enter_query: String,
    ime_enter_blocked: bool,
}

struct LoadingSemantics {
    label: String,
    progress: String,
}

struct FeedbackSemantics {
    defer_label: String,
    failed_label: String,
    running_label: String,
    completion_label: String,
    open_studio_target: String,
    open_studio_command: String,
    keyboard_path: String,
}

struct ActionPanelSemantics {
    actions: String,
    reader_label: String,
    open_studio_command: String,
}

fn result_semantics(state: &LauncherState) -> ResultSemantics {
    let a11y = AccessibilityContext::from_env();
    let selected_label = state
        .view
        .selected_result()
        .map(|result| {
            a11y.launcher_result_label(
                &result.action.name,
                &result.action.description,
                state.view.selected + 1,
                state.view.results.len(),
            )
        })
        .unwrap_or_else(|| "No matches".to_string());
    let selected_reader_label = selected_label.clone();
    let selected_position = if state.view.results.is_empty() {
        "0 of 0".to_string()
    } else {
        format!(
            "{} of {}",
            state.view.selected + 1,
            state.view.results.len()
        )
    };
    let selected_keycap = if state.view.results.is_empty() {
        "none".to_string()
    } else {
        input::launcher_result_keycap(0).unwrap_or_else(|| "none".to_string())
    };
    let selected_action_hint = state
        .view
        .preview
        .as_ref()
        .map(|preview| format!("{} {}", input::enter().label(), preview.primary_command))
        .unwrap_or_else(|| format!("{} none", input::enter().label()));
    ResultSemantics {
        search_reader_label: a11y.launcher_search_label(&state.view.query),
        result_group_label: a11y.launcher_result_group_label(i18n::translate(
            Locale::EnUs,
            "launcher.results.group.action_workflow",
        )),
        selected_label,
        selected_reader_label,
        selected_position,
        selected_keycap,
        selected_action_hint,
        action_bar_hint: format!(
            "{} {}",
            i18n::translate(Locale::EnUs, "launcher.action.actions"),
            input::launcher_action_panel().label()
        ),
    }
}

fn empty_query_semantics() -> EmptyQuerySemantics {
    let empty = LauncherState::new();
    let has_suggestions = !empty.view.results.is_empty();
    EmptyQuerySemantics {
        phase: format!("{:?}", empty.view.phase),
        mode: format!("{:?}", empty.view.result_mode),
        result_count: empty.view.results.len(),
        title: if has_suggestions {
            i18n::translate(Locale::EnUs, "launcher.results.suggested_workflows.title")
        } else {
            i18n::translate(Locale::EnUs, "launcher.empty.ready.title")
        }
        .to_string(),
        detail: i18n::translate(Locale::EnUs, "launcher.empty.ready.detail").to_string(),
    }
}

fn no_result_semantics() -> NoResultSemantics {
    let mut no_results = LauncherState::new();
    no_results.update_query("no-such-launcher-result");
    let mut no_results_enter = LauncherState::new();
    no_results_enter.update_query("no-such-launcher-result");
    no_results_enter.handle_keyboard_input(LauncherKey::Enter, false);
    let ime_enter_blocked = no_results
        .handle_keyboard_input(LauncherKey::Enter, true)
        .is_none()
        && no_results.view.feedback.is_none();
    NoResultSemantics {
        label: i18n::translate(Locale::EnUs, "launcher.empty.no_matches.title").to_string(),
        detail: i18n::translate(Locale::EnUs, "launcher.empty.no_matches.detail").to_string(),
        fallback: format!(
            "{} \"{}\"",
            i18n::translate(Locale::EnUs, "launcher.empty.ask_ai"),
            no_results.view.query
        ),
        phase: format!(
            "{:?}/{:?}",
            no_results.view.phase, no_results.view.result_mode
        ),
        enter_query: no_results_enter.view.query,
        ime_enter_blocked,
    }
}

fn loading_semantics() -> LoadingSemantics {
    let mut loading_state = LauncherState::new();
    loading_state.view.preview_searching("slow query");
    LoadingSemantics {
        label: i18n::translate(Locale::EnUs, "launcher.results.searching").to_string(),
        progress: format!(
            "{}px {} indeterminate",
            2,
            i18n::translate(Locale::EnUs, "launcher.results.searching.title")
        ),
    }
}

fn feedback_semantics() -> FeedbackSemantics {
    let defer_feedback = LauncherFeedback::from_execution(&deferred_execution());
    let failed_feedback = LauncherFeedback::from_execution(&failed_execution());
    let a11y = AccessibilityContext::from_env();
    let mut failed_state = LauncherState::new();
    failed_state.view.feedback = Some(failed_feedback.clone());
    let studio_intent = failed_state.open_studio_execution_history_from_feedback();
    let mut keyboard_state = LauncherState::new();
    keyboard_state.view.feedback = Some(failed_feedback.clone());
    keyboard_state.focus_section = crate::LauncherFocusSection::Feedback;
    keyboard_state.handle_keyboard_input(LauncherKey::ArrowDown, false);
    let retry = keyboard_state
        .view
        .selected_feedback_action()
        .map(|action| format!("{action:?}"))
        .unwrap_or_else(|| "none".to_string());
    keyboard_state.handle_keyboard_input(LauncherKey::ArrowDown, false);
    let open_studio = keyboard_state
        .view
        .selected_feedback_action()
        .map(|action| format!("{action:?}"))
        .unwrap_or_else(|| "none".to_string());
    let _ = keyboard_state.handle_keyboard_input(LauncherKey::Enter, false);
    FeedbackSemantics {
        defer_label: feedback_label(&defer_feedback),
        failed_label: feedback_label(&failed_feedback),
        running_label: a11y.launcher_running_label(&failed_feedback.action_name),
        completion_label: a11y.launcher_completed_label(&format!(
            "{} {}",
            defer_feedback.title, defer_feedback.detail
        )),
        open_studio_target: format!("{:?}", studio_intent.target),
        open_studio_command: studio_intent.command,
        keyboard_path: format!(
            "Feedback>{}:{retry}>{}:{open_studio}>{}:{}",
            input::arrow_down().label(),
            input::arrow_down().label(),
            input::enter().label(),
            keyboard_state
                .studio_intent
                .as_ref()
                .map(|intent| intent.command.as_str())
                .unwrap_or("none")
        ),
    }
}

fn action_panel_semantics(query: &str) -> ActionPanelSemantics {
    let mut state = LauncherState::new();
    state.update_query(query);
    state.open_action_panel();
    let actions = state
        .action_panel
        .items
        .iter()
        .map(|item| item.title())
        .collect::<Vec<_>>()
        .join(",");
    state.update_action_panel_query("studio");
    let _ = state.trigger_action_panel_selection();
    let selected_item = state
        .view
        .selected_result()
        .map(|result| result.action.name.as_str())
        .unwrap_or("No matches");
    ActionPanelSemantics {
        reader_label: AccessibilityContext::from_env()
            .launcher_action_panel_label(selected_item, state.action_panel.items.len()),
        open_studio_command: state
            .studio_intent
            .map(|intent| intent.command)
            .unwrap_or_else(|| "none".to_string()),
        actions,
    }
}

fn shortcut_help_semantics() -> String {
    crate::launcher_shortcut_help_summary()
}

fn deferred_execution() -> ActionExecution {
    ActionExecution {
        action_id: ActionId::default(),
        action_name: "StdFixtureTerminal".to_string(),
        status: ActionExecutionStatus::NeedsExternalRunner,
        message: "std-fixture-terminal".to_string(),
        output: Some(serde_json::json!({
            "deferred": true,
            "reason": "external runner action requires explicit user trigger",
        })),
        created_at: chrono::Utc::now(),
    }
}

fn failed_execution() -> ActionExecution {
    ActionExecution {
        action_id: ActionId::default(),
        action_name: "Plugin Crash".to_string(),
        status: ActionExecutionStatus::Failed,
        message: "plugin crashed while rendering launcher feedback".to_string(),
        output: None,
        created_at: chrono::Utc::now(),
    }
}

fn feedback_label(feedback: &LauncherFeedback) -> String {
    format!(
        "{}: {} {}",
        feedback.title, feedback.action_name, feedback.detail
    )
}
