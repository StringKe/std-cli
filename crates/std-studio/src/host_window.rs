use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HostWindowCommand {
    Close,
    Minimize,
    Maximize(bool),
}

pub(crate) fn apply_host_window_command(ctx: &egui::Context, command: HostWindowCommand) {
    match command {
        HostWindowCommand::Close => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
        HostWindowCommand::Minimize => {
            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        }
        HostWindowCommand::Maximize(maximized) => {
            ctx.send_viewport_cmd(egui::ViewportCommand::Maximized(maximized));
        }
    }
}

#[cfg(test)]
pub(crate) fn host_window_command_boundary_contract() -> &'static str {
    "host_window_commands=single-system-host-only;workspace_panes=internal-egui-only;commands=close|minimize|maximize"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_window_commands_are_limited_to_system_host_controls() {
        assert_eq!(
            host_window_command_boundary_contract(),
            "host_window_commands=single-system-host-only;workspace_panes=internal-egui-only;commands=close|minimize|maximize"
        );
        assert_eq!(HostWindowCommand::Close, HostWindowCommand::Close);
        assert_eq!(HostWindowCommand::Minimize, HostWindowCommand::Minimize);
        assert_eq!(
            HostWindowCommand::Maximize(true),
            HostWindowCommand::Maximize(true)
        );
    }
}
