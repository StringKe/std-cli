pub use crate::input_labels::{
    alt_modifier_label, named_key_label, primary_modifier_label, shift_modifier_label,
};

const IME_COMPOSING_ID: &str = "std-egui.ime-composing";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyBinding {
    Mod(char),
    ModShift(char),
    ModNamed(egui::Key),
    ModShiftNamed(egui::Key),
    ModOnlyNamed(egui::Key),
    AltNamed(egui::Key),
    Ctrl(char),
    ShiftNamed(egui::Key),
    Plain(egui::Key),
    Named(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImeActionGuard {
    pub composing: bool,
    pub frame_event: Option<String>,
    pub action_allowed: bool,
    pub contract: &'static str,
}

impl ImeActionGuard {
    pub fn blocks_actions(&self) -> bool {
        !self.action_allowed
    }
}

impl KeyBinding {
    pub fn label(self) -> String {
        match self {
            Self::Mod(key) => format!("{}+{}", primary_modifier_label(), key.to_ascii_uppercase()),
            Self::ModShift(key) => {
                format!(
                    "{}+{}+{}",
                    primary_modifier_label(),
                    shift_modifier_label(),
                    key.to_ascii_uppercase()
                )
            }
            Self::ModNamed(key) => format!("{}+{}", primary_modifier_label(), named_key_label(key)),
            Self::ModShiftNamed(key) => {
                format!(
                    "{}+{}+{}",
                    primary_modifier_label(),
                    shift_modifier_label(),
                    named_key_label(key)
                )
            }
            Self::ModOnlyNamed(key) => {
                format!("{}+{}", primary_modifier_label(), named_key_label(key))
            }
            Self::AltNamed(key) => format!("{}+{}", alt_modifier_label(), named_key_label(key)),
            Self::Ctrl(key) => format!("Ctrl+{}", key.to_ascii_uppercase()),
            Self::ShiftNamed(key) => format!("{}+{}", shift_modifier_label(), named_key_label(key)),
            Self::Plain(key) => named_key_label(key).to_string(),
            Self::Named(name) => name.to_string(),
        }
    }

    pub fn pressed(self, ctx: &egui::Context) -> bool {
        ctx.input(|input| match self {
            Self::Mod(key) => {
                input.modifiers.command && pressed_alpha(input, key) && !input.modifiers.shift
            }
            Self::ModShift(key) => {
                input.modifiers.command && input.modifiers.shift && pressed_alpha(input, key)
            }
            Self::ModNamed(key) => {
                input.modifiers.command && input.key_pressed(key) && !input.modifiers.shift
            }
            Self::ModShiftNamed(key) => {
                input.modifiers.command && input.modifiers.shift && input.key_pressed(key)
            }
            Self::ModOnlyNamed(key) => {
                input.modifiers.command
                    && !input.modifiers.shift
                    && !input.modifiers.alt
                    && !input.modifiers.ctrl
                    && input.key_pressed(key)
            }
            Self::AltNamed(key) => {
                !input.modifiers.command
                    && !input.modifiers.shift
                    && input.modifiers.alt
                    && !input.modifiers.ctrl
                    && input.key_pressed(key)
            }
            Self::Ctrl(key) => {
                !input.modifiers.command
                    && !input.modifiers.shift
                    && !input.modifiers.alt
                    && input.modifiers.ctrl
                    && pressed_alpha(input, key)
            }
            Self::ShiftNamed(key) => {
                !input.modifiers.command
                    && input.modifiers.shift
                    && !input.modifiers.alt
                    && !input.modifiers.ctrl
                    && input.key_pressed(key)
            }
            Self::Plain(key) => {
                !input.modifiers.command
                    && !input.modifiers.shift
                    && !input.modifiers.alt
                    && !input.modifiers.ctrl
                    && input.key_pressed(key)
            }
            Self::Named(_) => false,
        })
    }
}

pub fn launcher_action_panel() -> KeyBinding {
    KeyBinding::Mod('K')
}

pub fn launcher_cancel() -> KeyBinding {
    KeyBinding::Ctrl('C')
}

pub fn launcher_defer() -> KeyBinding {
    KeyBinding::ModShiftNamed(egui::Key::Enter)
}

pub fn launcher_open_studio() -> KeyBinding {
    KeyBinding::Mod('O')
}

pub fn launcher_copy_command() -> KeyBinding {
    KeyBinding::Mod('C')
}

pub fn launcher_result_keycap(index: usize) -> Option<String> {
    if index < 9 {
        Some(format!("{}+{}", primary_modifier_label(), index + 1))
    } else {
        None
    }
}

pub fn studio_command_palette() -> KeyBinding {
    KeyBinding::ModShift('P')
}

pub fn studio_command_palette_slash() -> KeyBinding {
    KeyBinding::ModNamed(egui::Key::Slash)
}

pub fn studio_new_workflow() -> KeyBinding {
    KeyBinding::Mod('N')
}

pub fn studio_zoom_reset() -> KeyBinding {
    KeyBinding::ModNamed(egui::Key::Num0)
}

pub fn studio_zoom_in() -> KeyBinding {
    KeyBinding::ModNamed(egui::Key::Equals)
}

pub fn studio_zoom_out() -> KeyBinding {
    KeyBinding::ModNamed(egui::Key::Minus)
}

pub fn studio_context_help() -> KeyBinding {
    KeyBinding::Plain(egui::Key::F1)
}

pub fn studio_quick_open() -> KeyBinding {
    KeyBinding::Mod('P')
}

pub fn studio_settings() -> KeyBinding {
    KeyBinding::ModNamed(egui::Key::Comma)
}

pub fn studio_sidebar_toggle() -> KeyBinding {
    KeyBinding::Mod('B')
}

pub fn studio_inspector_toggle() -> KeyBinding {
    KeyBinding::Mod('I')
}

pub fn studio_analysis_relation_toggle() -> KeyBinding {
    KeyBinding::Mod('L')
}

pub fn studio_analysis_qa_focus() -> KeyBinding {
    KeyBinding::Plain(egui::Key::Questionmark)
}

pub fn studio_workflow_step_move_up() -> KeyBinding {
    KeyBinding::AltNamed(egui::Key::ArrowUp)
}

pub fn studio_workflow_step_move_down() -> KeyBinding {
    KeyBinding::AltNamed(egui::Key::ArrowDown)
}

pub fn studio_workflow_test() -> KeyBinding {
    KeyBinding::ModOnlyNamed(egui::Key::Enter)
}

pub fn studio_workflow_simulate() -> KeyBinding {
    KeyBinding::ModShiftNamed(egui::Key::Enter)
}

pub fn studio_workflow_save() -> KeyBinding {
    KeyBinding::Mod('S')
}

pub fn studio_workflow_history() -> KeyBinding {
    KeyBinding::ModShift('H')
}

pub fn studio_bottom_panel_toggle() -> KeyBinding {
    KeyBinding::Mod('J')
}

pub fn studio_previous_bottom_panel_tab() -> KeyBinding {
    KeyBinding::ModNamed(egui::Key::ArrowLeft)
}

pub fn studio_next_bottom_panel_tab() -> KeyBinding {
    KeyBinding::ModNamed(egui::Key::ArrowRight)
}

pub fn studio_close_tab() -> KeyBinding {
    KeyBinding::Mod('W')
}

pub fn studio_previous_workspace_pane() -> KeyBinding {
    KeyBinding::ModShiftNamed(egui::Key::ArrowUp)
}

pub fn studio_next_workspace_pane() -> KeyBinding {
    KeyBinding::ModShiftNamed(egui::Key::ArrowDown)
}

pub fn escape() -> KeyBinding {
    KeyBinding::Plain(egui::Key::Escape)
}

pub fn enter() -> KeyBinding {
    KeyBinding::Plain(egui::Key::Enter)
}

pub fn tab() -> KeyBinding {
    KeyBinding::Plain(egui::Key::Tab)
}

pub fn shift_tab() -> KeyBinding {
    KeyBinding::ShiftNamed(egui::Key::Tab)
}

pub fn arrow_down() -> KeyBinding {
    KeyBinding::Plain(egui::Key::ArrowDown)
}

pub fn arrow_up() -> KeyBinding {
    KeyBinding::Plain(egui::Key::ArrowUp)
}

pub fn mod_arrow_down() -> KeyBinding {
    KeyBinding::ModNamed(egui::Key::ArrowDown)
}

pub fn mod_arrow_up() -> KeyBinding {
    KeyBinding::ModNamed(egui::Key::ArrowUp)
}

pub fn launcher_delete_previous_token() -> KeyBinding {
    KeyBinding::ModNamed(egui::Key::Backspace)
}

pub fn pressed_mod_number(ctx: &egui::Context, max: usize) -> Option<usize> {
    const KEYS: [egui::Key; 9] = [
        egui::Key::Num1,
        egui::Key::Num2,
        egui::Key::Num3,
        egui::Key::Num4,
        egui::Key::Num5,
        egui::Key::Num6,
        egui::Key::Num7,
        egui::Key::Num8,
        egui::Key::Num9,
    ];
    ctx.input(|input| {
        if !input.modifiers.command || input.modifiers.shift {
            return None;
        }
        KEYS.iter()
            .take(max.min(KEYS.len()))
            .position(|key| input.key_pressed(*key))
    })
}

pub fn ime_composing(ctx: &egui::Context) -> bool {
    let frame_state = ctx.input(ime_composing_from_events);
    ctx.data_mut(|data| {
        let previous = data
            .get_temp::<bool>(egui::Id::new(IME_COMPOSING_ID))
            .unwrap_or(false);
        let current = frame_state.unwrap_or(previous);
        data.insert_temp(egui::Id::new(IME_COMPOSING_ID), current);
        current
    })
}

pub fn ime_frame_event(ctx: &egui::Context) -> Option<egui::ImeEvent> {
    ctx.input(|input| {
        input.events.iter().find_map(|event| match event {
            egui::Event::Ime(event) => Some(event.clone()),
            _ => None,
        })
    })
}

pub fn ime_action_guard(ctx: &egui::Context) -> ImeActionGuard {
    let frame_event = ime_frame_event(ctx).map(ime_event_label);
    let composing = ime_composing(ctx);
    ImeActionGuard {
        composing,
        frame_event,
        action_allowed: !composing,
        contract: ime_action_guard_contract(),
    }
}

pub fn ime_action_guard_contract() -> &'static str {
    "ime-action-guard=preedit-blocks-enter-escape-arrows-shortcuts;commit-restores-actions"
}

fn ime_composing_from_events(input: &egui::InputState) -> Option<bool> {
    input.events.iter().fold(None, |state, event| match event {
        egui::Event::Ime(egui::ImeEvent::Preedit(_)) => Some(true),
        egui::Event::Ime(egui::ImeEvent::Commit(_))
        | egui::Event::Ime(egui::ImeEvent::Disabled) => Some(false),
        _ => state,
    })
}

fn ime_event_label(event: egui::ImeEvent) -> String {
    match event {
        egui::ImeEvent::Enabled => "enabled".to_string(),
        egui::ImeEvent::Preedit(value) => format!("preedit:{value}"),
        egui::ImeEvent::Commit(value) => format!("commit:{value}"),
        egui::ImeEvent::Disabled => "disabled".to_string(),
    }
}

fn pressed_alpha(input: &egui::InputState, key: char) -> bool {
    let key = match key.to_ascii_uppercase() {
        'B' => egui::Key::B,
        'C' => egui::Key::C,
        'I' => egui::Key::I,
        'J' => egui::Key::J,
        'K' => egui::Key::K,
        'L' => egui::Key::L,
        'H' => egui::Key::H,
        'N' => egui::Key::N,
        'O' => egui::Key::O,
        'P' => egui::Key::P,
        'S' => egui::Key::S,
        'W' => egui::Key::W,
        _ => return false,
    };
    input.key_pressed(key)
}

#[cfg(test)]
#[path = "input_tests.rs"]
mod tests;
