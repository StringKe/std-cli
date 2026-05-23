use crate::CliError;

pub(crate) fn verify_capture_run_id(value: &str) -> Result<(), CliError> {
    let valid = value.len() >= 18
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'));
    if valid {
        return Ok(());
    }
    Err(CliError::Doctor(format!(
        "capture manifest run_id must be a stable current-run token: {value}"
    )))
}

pub(crate) fn verify_capture_line_run_id(
    line: &str,
    expected: &str,
    surface: &str,
    theme: &str,
    scenario: &str,
) -> Result<(), CliError> {
    let actual = capture_field(line, "run_id=")?;
    if actual == expected {
        return Ok(());
    }
    Err(CliError::Doctor(format!(
        "capture manifest run_id mismatch for {surface} {theme} {scenario}: expected={expected} actual={actual}"
    )))
}

fn capture_field<'a>(line: &'a str, key: &str) -> Result<&'a str, CliError> {
    line.split_whitespace()
        .find_map(|part| part.strip_prefix(key))
        .ok_or_else(|| CliError::Doctor(format!("capture manifest field missing: {key}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_run_id_accepts_script_token() {
        verify_capture_run_id("20260523T120000Z-12345").unwrap();
    }

    #[test]
    fn capture_run_id_rejects_weak_or_shell_sensitive_value() {
        assert!(verify_capture_run_id("old").is_err());
        assert!(verify_capture_run_id("2026-05-23T12:00:00Z;rm").is_err());
    }

    #[test]
    fn capture_line_run_id_must_match_header() {
        let line = "launcher theme=dark scenario=results run_id=run-123";

        verify_capture_line_run_id(line, "run-123", "launcher", "dark", "results").unwrap();
        assert!(
            verify_capture_line_run_id(line, "run-456", "launcher", "dark", "results")
                .unwrap_err()
                .to_string()
                .contains("run_id mismatch")
        );
    }
}
