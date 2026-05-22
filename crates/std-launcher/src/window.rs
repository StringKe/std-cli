use eframe::egui;
use std_launcher::LauncherWindowCommand;

const WINDOW_VERTICAL_ANCHOR: f32 = 0.28;

pub(crate) fn apply_window_commands(
    ctx: &egui::Context,
    commands: &[LauncherWindowCommand],
    viewport_size: egui::Vec2,
) {
    for command in commands {
        match command {
            LauncherWindowCommand::PositionForPanel => {
                if let Some(position) = launcher_window_position(ctx, viewport_size) {
                    ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(position));
                }
            }
            LauncherWindowCommand::ResizeToPanel => {
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(viewport_size));
            }
            LauncherWindowCommand::ResizeToHiddenHost => {
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(hidden_host_size()));
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
    viewport_size: egui::Vec2,
) -> Option<egui::Pos2> {
    let monitor_size = ctx.input(|input| input.viewport().monitor_size)?;
    Some(launcher_window_position_for_monitor(
        monitor_size,
        viewport_size,
    ))
}

fn launcher_window_position_for_monitor(
    monitor_size: egui::Vec2,
    viewport_size: egui::Vec2,
) -> egui::Pos2 {
    let x = ((monitor_size.x - viewport_size.x) * 0.5).max(0.0);
    let y =
        (monitor_size.y * WINDOW_VERTICAL_ANCHOR).min((monitor_size.y - viewport_size.y).max(0.0));
    egui::pos2(x, y)
}

pub(crate) fn hidden_host_size() -> egui::Vec2 {
    egui::vec2(1.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launcher_window_position_anchors_native_window_to_spec_region() {
        let position = launcher_window_position_for_monitor(
            egui::vec2(1440.0, 900.0),
            egui::vec2(720.0, 64.0),
        );

        assert_eq!(position, egui::pos2(360.0, 252.0));
    }

    #[test]
    fn launcher_window_host_contract_forbids_visible_host_gap_positioning() {
        assert_eq!(
            std_launcher::launcher_host_positioning_contract(),
            "host_positioning=show:resize-to-transparent-host>outer-position-0.28-monitor-anchor>visible>focus;hide:resize-to-1x1>hidden;native_host=transparent;panel_surface=opaque-bg-surface-0;host_background=none;host_gutter=16px"
        );
    }

    #[test]
    fn launcher_hidden_host_collapses_to_one_pixel() {
        assert_eq!(hidden_host_size(), egui::vec2(1.0, 1.0));
    }

    #[test]
    fn launcher_window_position_clamps_when_panel_is_taller_than_monitor() {
        let position = launcher_window_position_for_monitor(
            egui::vec2(800.0, 240.0),
            egui::vec2(720.0, 320.0),
        );

        assert_eq!(position, egui::pos2(40.0, 0.0));
    }
}
