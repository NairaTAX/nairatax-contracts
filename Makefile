.PHONY: build test fmt fmt-check clippy check clean

build:
	./scripts/build.sh

test:
	cargo test

fmt:
	cargo fmt

fmt-check:
	cargo fmt --check

clippy:
	cargo clippy --all-targets -- -D warnings

check: fmt-check clippy test

clean:
	cargo clean
