pub(crate) fn blocked_desktop_command_reason(program: &str, args: &[String]) -> Option<String> {
    let command = std::iter::once(program)
        .chain(args.iter().map(String::as_str))
        .collect::<Vec<_>>()
        .join(" ");
    if desktop_command_blocked(program, &command) {
        Some(format!("STD_TEST_MODE blocked desktop command: {command}"))
    } else {
        None
    }
}

fn desktop_command_blocked(program: &str, command: &str) -> bool {
    if !crate::std_test_mode_enabled() {
        return false;
    }
    let lower_program = program.to_ascii_lowercase();
    let lower_command = command.to_ascii_lowercase();
    blocked_desktop_program(&lower_program) || blocked_desktop_target(command, &lower_command)
}

fn blocked_desktop_program(program: &str) -> bool {
    blocked_program_terms()
        .iter()
        .any(|term| program == term.as_str())
}

fn blocked_desktop_target(command: &str, lower_command: &str) -> bool {
    blocked_target_terms()
        .iter()
        .any(|term| lower_command.contains(term))
        || command.contains(&["微", "信"].concat())
}

fn blocked_program_terms() -> Vec<String> {
    vec![
        ["op", "en"].concat(),
        ["/usr/bin/", "op", "en"].concat(),
        ["osa", "script"].concat(),
        ["/usr/bin/", "osa", "script"].concat(),
        "screencapture".to_string(),
        "/usr/sbin/screencapture".to_string(),
    ]
}

fn blocked_target_terms() -> Vec<String> {
    vec![
        ["1", "password"].concat(),
        ["one", "password"].concat(),
        ["we", "chat"].concat(),
        ["wei", "xin"].concat(),
        ["we", "chat://"].concat(),
        ["wei", "xin://"].concat(),
        ["op", "en -a terminal"].concat(),
        ["tell applic", "ation"].concat(),
        ["system", " events"].concat(),
        ["/applic", "ations/"].concat(),
        ["/system/applic", "ations"].concat(),
    ]
}
