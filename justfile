set shell := ["bash", "-uc"]

nightly := `rustc --version | grep -oE '[0-9]{4}-[0-9]{2}-[0-9]{2}' | sed 's/^/nightly-/'`

check:
    cargo check --all-targets

fix:
    cargo fix --all-targets --allow-dirty --allow-staged

fmt:
    cargo +{{ nightly }} fmt
    RUST_LOG=error taplo fmt

fmt-check:
    cargo +{{ nightly }} fmt --check

lint:
    cargo clippy --all-targets --no-deps -- -D warnings

lint-fix:
    cargo clippy --all-targets --no-deps --allow-dirty --allow-staged --fix

test:
    cargo test

doc:
    cargo doc --no-deps

all: check fmt lint test doc

public-deps:
    python3 {{ env("PUBLIC_DEPS_CHECK", "../public-deps-check/public_deps_check.py") }} analyze --base origin/main --head HEAD --toolchain {{ nightly }}
