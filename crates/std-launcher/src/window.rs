use eframe::egui;
use std_launcher::LauncherWindowCommand;

pub(crate) fn apply_window_commands(ctx: &egui::Context, commands: &[LauncherWindowCommand]) {
    for command in commands {
        match command {
            LauncherWindowCommand::SetVisible(visible) => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Visible(*visible));
            }
            LauncherWindowCommand::Focus => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            }
        }
    }
}
