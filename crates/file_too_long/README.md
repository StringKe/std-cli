# file_too_long

`file_too_long` is a local Dylint lint for this workspace. It enforces the project source-file limit for Rust code.

## Rule

Rust source files must be at most 500 lines. Files above the limit fail the Dylint quality gate when run with `DYLINT_RUSTFLAGS="-D warnings"`.

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

This lint only covers Rust source files. Function size, argument count, and complexity remain Clippy rules configured in `clippy.toml`.
