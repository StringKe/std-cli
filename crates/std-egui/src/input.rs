#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyBinding {
    Mod(char),
    ModShift(char),
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
        'K' => egui::Key::K,
        'P' => egui::Key::P,
        _ => return false,
    };
    input.key_pressed(key)
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
    }

    #[test]
    fn ime_guard_api_is_available_to_ui_surfaces() {
        let ctx = egui::Context::default();

        assert!(!ime_composing(&ctx));
    }
}
