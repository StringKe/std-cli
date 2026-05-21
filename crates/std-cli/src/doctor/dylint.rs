use crate::{
    doctor::workspace::{check_text, read_required},
    CliError,
};
use std::path::Path;

pub(crate) fn check_dylint_lint(root: &Path) -> Result<(), CliError> {
    let lint = read_required(&root.join("crates/file_too_long/src/lib.rs"))?;
    for required in [
        "const MAX_SOURCE_FILE_LINES: usize = 500;",
        "NO_INLINE_VISUAL_VALUES",
        "Color32::from_rgb",
        "Color32::from_rgba_premultiplied",
        "Color32::from_black_alpha",
        "std-egui tokens",
        "TOKEN_PALETTE_PATH",
    ] {
        check_text(&lint, required)?;
    }
    let fixture = read_required(&root.join("crates/file_too_long/ui/inline_visual_color.rs"))?;
    check_text(&fixture, "egui::Color32::from_rgb")?;
    let expected = read_required(&root.join("crates/file_too_long/ui/inline_visual_color.stderr"))?;
    check_text(
        &expected,
        "inline Color32 constructor must use token palette",
    )?;
    check_text(
        &read_required(&root.join("crates/file_too_long/Cargo.toml"))?,
        "dylint_linting = \"5.0.0\"",
    )?;
    check_text(
        &read_required(&root.join("crates/file_too_long/rust-toolchain"))?,
        "channel = \"nightly-2025-09-18\"",
    )?;
    check_text(
        &read_required(&root.join("crates/file_too_long/.cargo/config.toml"))?,
        "dylint-link",
    )
}
