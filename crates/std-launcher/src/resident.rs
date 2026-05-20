use tray_icon::{
    menu::{Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem},
    Icon, MouseButton, MouseButtonState, TrayIcon, TrayIconBuilder, TrayIconEvent,
};

const SHOW_ID: &str = "std-launcher-show";
const HIDE_ID: &str = "std-launcher-hide";
const QUIT_ID: &str = "std-launcher-quit";

pub(crate) struct ResidentEntry {
    _tray_icon: TrayIcon,
    show_item: MenuItem,
    hide_item: MenuItem,
    quit_item: MenuItem,
    status: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResidentCommand {
    Show,
    Hide,
    Quit,
}

impl ResidentEntry {
    pub(crate) fn new() -> Result<Self, String> {
        let menu = Menu::new();
        let show_item = MenuItem::with_id(MenuId::new(SHOW_ID), "Show Launcher", true, None);
        let hide_item = MenuItem::with_id(MenuId::new(HIDE_ID), "Hide Launcher", true, None);
        let quit_item = MenuItem::with_id(MenuId::new(QUIT_ID), "Quit std Launcher", true, None);
        menu.append_items(&[
            &show_item,
            &hide_item,
            &PredefinedMenuItem::separator(),
            &quit_item,
        ])
        .map_err(|error| error.to_string())?;

        let tray_icon = TrayIconBuilder::new()
            .with_tooltip("std Launcher")
            .with_title("std")
            .with_icon(tray_icon())
            .with_icon_as_template(true)
            .with_menu(Box::new(menu))
            .build()
            .map_err(|error| error.to_string())?;

        Ok(Self {
            _tray_icon: tray_icon,
            show_item,
            hide_item,
            quit_item,
            status: "menu bar ready".to_string(),
        })
    }

    pub(crate) fn status(&self) -> &str {
        &self.status
    }

    pub(crate) fn poll_command(&self) -> Option<ResidentCommand> {
        if let Ok(event) = TrayIconEvent::receiver().try_recv() {
            if tray_click_shows(&event) {
                return Some(ResidentCommand::Show);
            }
        }
        while let Ok(event) = MenuEvent::receiver().try_recv() {
            if event.id == *self.show_item.id() {
                return Some(ResidentCommand::Show);
            }
            if event.id == *self.hide_item.id() {
                return Some(ResidentCommand::Hide);
            }
            if event.id == *self.quit_item.id() {
                return Some(ResidentCommand::Quit);
            }
        }
        None
    }
}

fn tray_click_shows(event: &TrayIconEvent) -> bool {
    matches!(
        event,
        TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
        }
    )
}

fn tray_icon() -> Icon {
    let width = 16;
    let height = 16;
    let mut rgba = Vec::with_capacity(width * height * 4);
    for y in 0..height {
        for x in 0..width {
            let inside = x == 7 || x == 8 || y == 7 || y == 8 || x == y || x + y == 15;
            let alpha = if inside { 255 } else { 0 };
            rgba.extend_from_slice(&[0, 0, 0, alpha]);
        }
    }
    Icon::from_rgba(rgba, width as u32, height as u32).expect("static tray icon is valid")
}
