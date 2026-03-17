.PHONY: build check fmt clippy test clean publish

build:
	cargo build-sbf

check:
	cargo check --all-features

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-features -- -D warnings

test:
	cargo test --all-features

clean:
	cargo clean

publish:
	cargo publish --dry-run
	@echo "Run 'cargo publish' to publish to crates.io"

.DEFAULT_GOAL := check
