#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyBinding {
    Mod(char),
    ModShift(char),
    ModNamed(egui::Key),
    ModShiftNamed(egui::Key),
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

pub fn escape() -> KeyBinding {
    KeyBinding::Plain(egui::Key::Escape)
}

pub fn enter() -> KeyBinding {
    KeyBinding::Plain(egui::Key::Enter)
}

pub fn arrow_down() -> KeyBinding {
    KeyBinding::Plain(egui::Key::ArrowDown)
}

pub fn arrow_up() -> KeyBinding {
    KeyBinding::Plain(egui::Key::ArrowUp)
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
        egui::Key::Comma => ",",
        egui::Key::Enter => "Enter",
        egui::Key::Escape => "Esc",
        egui::Key::Slash => "/",
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
    }

    #[test]
    fn ime_guard_api_is_available_to_ui_surfaces() {
        let ctx = egui::Context::default();

        assert!(!ime_composing(&ctx));
    }
}
