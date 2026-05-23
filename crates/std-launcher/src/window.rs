use eframe::egui;
use std_egui::tokens::LauncherSize;
use std_launcher::LauncherWindowCommand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LauncherHostWindowCommand {
    CancelClose,
    Close,
    SyncInnerSize,
}

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

pub(crate) fn apply_host_window_command(
    ctx: &egui::Context,
    command: LauncherHostWindowCommand,
    viewport_size: egui::Vec2,
) {
    match command {
        LauncherHostWindowCommand::CancelClose => {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
        }
        LauncherHostWindowCommand::Close => {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
        LauncherHostWindowCommand::SyncInnerSize => {
            ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(viewport_size));
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
    LauncherSize::panel_position_for_monitor(monitor_size, viewport_size)
}

pub(crate) fn hidden_host_size() -> egui::Vec2 {
    LauncherSize::hidden_host_size()
}

#[cfg(test)]
pub(crate) fn launcher_host_window_command_boundary_contract() -> &'static str {
    "launcher_host_window_commands=single-transparent-host-only;commands=cancel-close|close|sync-inner-size;launcher_window_commands=show|hide|focus|resize"
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
    fn launcher_window_host_contract_uses_transparent_host_gutter() {
        assert_eq!(
            std_launcher::launcher_host_positioning_contract(),
            "host_positioning=show:resize-to-panel>outer-position-0.28-monitor-anchor>visible>focus;hide:resize-to-1x1>hidden;native_host=panel-sized-transparent;panel_surface=opaque-bg-surface-0;host_background=none;host_gutter=0px"
        );
    }

    #[test]
    fn launcher_host_window_commands_are_limited_to_system_host_controls() {
        assert_eq!(
            launcher_host_window_command_boundary_contract(),
            "launcher_host_window_commands=single-transparent-host-only;commands=cancel-close|close|sync-inner-size;launcher_window_commands=show|hide|focus|resize"
        );
        assert_eq!(
            LauncherHostWindowCommand::CancelClose,
            LauncherHostWindowCommand::CancelClose
        );
        assert_eq!(
            LauncherHostWindowCommand::Close,
            LauncherHostWindowCommand::Close
        );
        assert_eq!(
            LauncherHostWindowCommand::SyncInnerSize,
            LauncherHostWindowCommand::SyncInnerSize
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
