.PHONY: install build run clean uninstall help sync-version test tauri-dev tauri-build

# Default target
help:
	@echo "1gh - One Good Hour focus timer"
	@echo ""
	@echo "Available targets:"
	@echo "  make install    - Install 1gh to ~/.cargo/bin (system-wide)"
	@echo "  make build      - Build release binary to target/release/1gh"
	@echo "  make run        - Build and run in release mode"
	@echo "  make test       - Run all tests"
	@echo "  make clean      - Remove build artifacts"
	@echo "  make uninstall  - Remove installed binary"
	@echo "  make sync-version - Sync version from Cargo.toml to docs and packaging"
	@echo "  make tauri-dev  - Run Tauri desktop app in dev mode"
	@echo "  make tauri-build - Build Tauri desktop app"

VERSION := $(shell grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')

sync-version:
	@echo "Syncing version $(VERSION) from Cargo.toml"
	sed -i '' 's|<span>v[^<]*</span>|<span>v$(VERSION)</span>|' docs/index.html
	sed -i '' 's|"version": "[^"]*"|"version": "$(VERSION)"|' src-tauri/tauri.conf.json

install:
	cargo install --path crates/one-good-hour-tui

build: sync-version
	cargo build --release -p one-good-hour-tui

run:
	cargo run --release -p one-good-hour-tui

test:
	cargo test --workspace

clean:
	cargo clean

uninstall:
	cargo uninstall one-good-hour-tui

tauri-dev:
	cargo tauri dev

tauri-build:
	cargo tauri build
