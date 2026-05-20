use eframe::egui;
use std_launcher::LauncherWindowCommand;

pub(crate) fn apply_window_commands(
    ctx: &egui::Context,
    commands: &[LauncherWindowCommand],
    panel_size: egui::Vec2,
) {
    for command in commands {
        match command {
            LauncherWindowCommand::PositionForPanel => {
                if let Some(position) = launcher_window_position(ctx, panel_size) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(position));
                }
            }
            LauncherWindowCommand::ResizeToPanel => {
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(panel_size));
            }
            LauncherWindowCommand::SetVisible(visible) => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Visible(*visible));
            }
            LauncherWindowCommand::Focus => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            }
        }
    }
}

pub(crate) fn launcher_window_position(
    ctx: &egui::Context,
    panel_size: egui::Vec2,
) -> Option<egui::Pos2> {
    let monitor_size = ctx.input(|input| input.viewport().monitor_size)?;
    Some(launcher_window_position_for_monitor(
        monitor_size,
        panel_size,
    ))
}

fn launcher_window_position_for_monitor(
    monitor_size: egui::Vec2,
    panel_size: egui::Vec2,
) -> egui::Pos2 {
    let x = ((monitor_size.x - panel_size.x) * 0.5).max(0.0);
    let y = (monitor_size.y * 0.28).min((monitor_size.y - panel_size.y).max(0.0));
    egui::pos2(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launcher_window_position_anchors_native_window_to_spec_region() {
        let position = launcher_window_position_for_monitor(
            egui::vec2(1440.0, 900.0),
            egui::vec2(744.0, 88.0),
        );

        assert_eq!(position, egui::pos2(348.0, 252.0));
    }

    #[test]
    fn launcher_window_position_clamps_when_panel_is_taller_than_monitor() {
        let position = launcher_window_position_for_monitor(
            egui::vec2(800.0, 240.0),
            egui::vec2(744.0, 320.0),
        );

        assert_eq!(position, egui::pos2(28.0, 0.0));
    }
}
