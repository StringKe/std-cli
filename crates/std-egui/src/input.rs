#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyBinding {
    Mod(char),
    ModShift(char),
    ModNamed(egui::Key),
    ModShiftNamed(egui::Key),
    ShiftNamed(egui::Key),
    Plain(egui::Key),
    Named(&'static str),
}

impl KeyBinding {
    pub fn label(self) -> String {
        match self {
            Self::Mod(key) => format!("{}+{}", primary_modifier_label(), key.to_ascii_uppercase()),
            Self::ModShift(key) => {
                format!(
                    "{}+Shift+{}",
                    primary_modifier_label(),
                    key.to_ascii_uppercase()
                )
            }
            Self::ModNamed(key) => format!("{}+{}", primary_modifier_label(), named_key_label(key)),
            Self::ModShiftNamed(key) => {
                format!(
                    "{}+Shift+{}",
                    primary_modifier_label(),
                    named_key_label(key)
                )
            }
            Self::ShiftNamed(key) => format!("Shift+{}", named_key_label(key)),
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

pub fn primary_modifier_label() -> &'static str {
    if cfg!(target_os = "macos") {
        "Cmd"
    } else {
        "Ctrl"
    }
}

pub fn launcher_action_panel() -> KeyBinding {
    KeyBinding::Mod('K')
}

pub fn studio_command_palette() -> KeyBinding {
    KeyBinding::ModShift('P')
}

pub fn studio_command_palette_slash() -> KeyBinding {
    KeyBinding::ModNamed(egui::Key::Slash)
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

pub fn studio_bottom_panel_toggle() -> KeyBinding {
    KeyBinding::Mod('J')
}

pub fn studio_close_tab() -> KeyBinding {
    KeyBinding::Mod('W')
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
    ctx.input(|input| {
        input.events.iter().any(|event| {
            matches!(
                event,
                egui::Event::Ime(egui::ImeEvent::Enabled)
                    | egui::Event::Ime(egui::ImeEvent::Preedit(_))
            )
        })
    })
}

fn pressed_alpha(input: &egui::InputState, key: char) -> bool {
    let key = match key.to_ascii_uppercase() {
        'B' => egui::Key::B,
        'I' => egui::Key::I,
        'J' => egui::Key::J,
        'K' => egui::Key::K,
        'P' => egui::Key::P,
        _ => return false,
    };
    input.key_pressed(key)
}

fn named_key_label(key: egui::Key) -> &'static str {
    match key {
        egui::Key::ArrowDown => "Down",
        egui::Key::ArrowUp => "Up",
        egui::Key::Backspace => "Backspace",
        egui::Key::Comma => ",",
        egui::Key::Enter => "Enter",
        egui::Key::Escape => "Esc",
        egui::Key::Slash => "/",
        egui::Key::Tab => "Tab",
        _ => "Key",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keybinding_labels_use_platform_primary_modifier() {
        let label = launcher_action_panel().label();

        assert!(label == "Cmd+K" || label == "Ctrl+K");
    }

    #[test]
    fn studio_palette_binding_matches_docs() {
        assert!(studio_command_palette().label().ends_with("+P"));
        assert!(studio_command_palette_slash().label().ends_with("+/"));
        assert!(studio_quick_open().label().ends_with("+P"));
        assert!(studio_settings().label().ends_with("+,"));
        assert!(launcher_delete_previous_token()
            .label()
            .ends_with("+Backspace"));
        assert_eq!(tab().label(), "Tab");
        assert_eq!(shift_tab().label(), "Shift+Tab");
    }

    #[test]
    fn ime_guard_api_is_available_to_ui_surfaces() {
        let ctx = egui::Context::default();

        assert!(!ime_composing(&ctx));
    }
}
