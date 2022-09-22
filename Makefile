all: lint unit-tests build-release

build-debug:
	cargo build

build-release:
	cargo build --release --all-features

unit-tests:
	cargo test -- --nocapture

lint:
	cargo fmt --all -- --check && cargo clippy -- -D warnings

.PHONY: all build-debug build-release unit-tests lint