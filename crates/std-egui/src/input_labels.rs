pub fn primary_modifier_label() -> &'static str {
    if cfg!(target_os = "macos") {
        "⌘"
    } else {
        "Ctrl"
    }
}

pub fn alt_modifier_label() -> &'static str {
    if cfg!(target_os = "macos") {
        "⌥"
    } else {
        "Alt"
    }
}

pub fn shift_modifier_label() -> &'static str {
    if cfg!(target_os = "macos") {
        "⇧"
    } else {
        "Shift"
    }
}

pub fn named_key_label(key: egui::Key) -> &'static str {
    match key {
        egui::Key::ArrowDown => "↓",
        egui::Key::ArrowLeft => "Left",
        egui::Key::ArrowRight => "Right",
        egui::Key::ArrowUp => "↑",
        egui::Key::Backspace => {
            if cfg!(target_os = "macos") {
                "⌫"
            } else {
                "Backspace"
            }
        }
        egui::Key::Comma => ",",
        egui::Key::Enter => {
            if cfg!(target_os = "macos") {
                "↵"
            } else {
                "Enter"
            }
        }
        egui::Key::Escape => "Esc",
        egui::Key::Questionmark => "?",
        egui::Key::Slash => "/",
        egui::Key::Tab => {
            if cfg!(target_os = "macos") {
                "⇥"
            } else {
                "Tab"
            }
        }
        _ => "Key",
    }
}
