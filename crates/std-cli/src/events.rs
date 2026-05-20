use crate::CliError;
use std_core::{EventBus, StdCore};

pub(crate) fn format_events(core: &StdCore, audit: bool) -> Result<String, CliError> {
    let events = if audit {
        core.read_audit_events()?
    } else {
        core.events()?
    };
    let lines = events
        .into_iter()
        .map(|event| {
            format!(
                "{}\t{:?}\t{}",
                event.created_at.to_rfc3339(),
                event.event_type,
                event.source
            )
        })
        .collect::<Vec<_>>();
    Ok(lines.join("\n"))
}
