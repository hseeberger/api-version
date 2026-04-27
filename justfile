set shell := ["bash", "-uc"]

nightly := `rustc --version | grep -oE '[0-9]{4}-[0-9]{2}-[0-9]{2}' | sed 's/^/nightly-/'`

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
