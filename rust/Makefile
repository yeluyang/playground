.PHONY: all
all: build

.PHONY: check
check:
	cargo fmt --all -- -l --check
	cargo clippy
	cargo check

.PHONY: debug
debug:
	cargo build

.PHONY: build
build: check
	cargo build --release

.PHONY: test
test:
	cargo test

.PHONY: clean
clean:
	cargo clean
