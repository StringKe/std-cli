.PHONY: quality fmt clippy dylint dylint-test test deny machete

quality: fmt clippy dylint dylint-test test deny machete

fmt:
	cargo fmt --all --check

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

dylint:
	DYLINT_RUSTFLAGS="-D warnings" cargo dylint --workspace --all -- --all-targets

dylint-test:
	cargo +nightly-2025-09-18 test --manifest-path crates/file_too_long/Cargo.toml

test:
	cargo test --workspace -- --test-threads=1

deny:
	cargo deny check

machete:
	cargo machete
