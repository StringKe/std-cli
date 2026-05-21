# file_too_long

`file_too_long` is a local Dylint lint crate for this workspace. It enforces Rust source-file limits and UI visual token boundaries.

## Rule

- `file_too_long`: Rust source files must be at most 500 lines. Files above the limit fail the Dylint quality gate when run with `DYLINT_RUSTFLAGS="-D warnings"`.
- `no_inline_visual_values`: product UI code must not call `Color32::from_rgb` or `Color32::from_rgba_*` directly. Visual colors must come from `std-egui::tokens`; `std-egui/src/tokens/palette.rs` is the only RGB/RGBA definition boundary.

## Usage

From the workspace root:

```bash
DYLINT_RUSTFLAGS="-D warnings" cargo dylint --workspace --all -- --all-targets
```

The lint crate itself is tested with:

```bash
cargo +nightly-2025-09-18 test --manifest-path crates/file_too_long/Cargo.toml
```

## Scope

This lint crate only covers Rust source files and inline UI color constructors. Function size, argument count, and complexity remain Clippy rules configured in `clippy.toml`.
