set shell := ["bash", "-uc"]

rust_version := `grep channel rust-toolchain.toml | sed -r 's/channel = "(.*)"/\1/'`
nightly := "nightly-2025-12-08"

check:
	cargo check --tests

fmt:
    cargo +{{nightly}} fmt

fmt-check:
    cargo +{{nightly}} fmt --check

fix:
	cargo fix --tests --allow-dirty --allow-staged

lint:
	cargo clippy --tests --no-deps -- -D warnings

lint-fix:
	cargo clippy --tests --no-deps --allow-dirty --allow-staged --fix

test:
	cargo test --tests

doc:
	cargo doc --no-deps

all: check fmt lint test doc
