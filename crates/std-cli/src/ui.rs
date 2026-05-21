use clap::Subcommand;

use background::{background_smoke, BackgroundSmokeConfig};

mod background;

#[derive(Debug, Subcommand)]
pub enum UiCommand {
    /// Run the opt-in-only background UI acceptance smoke gate.
    BackgroundSmoke {
        #[arg(long)]
        harness_pid: Option<u32>,
        #[arg(long)]
        window_id: Option<u32>,
        #[arg(long)]
        bundle_id: Option<String>,
        #[arg(long)]
        window_title: Option<String>,
    },
}

pub(crate) fn handle_ui(command: UiCommand) -> String {
    match command {
        UiCommand::BackgroundSmoke {
            harness_pid,
            window_id,
            bundle_id,
            window_title,
        } => background_smoke(BackgroundSmokeConfig {
            harness_pid,
            window_id,
            bundle_id,
            window_title,
        }),
    }
}
