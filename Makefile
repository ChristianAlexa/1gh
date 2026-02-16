.PHONY: install build run clean uninstall help sync-version

# Default target
help:
	@echo "1gh - One Good Hour focus timer"
	@echo ""
	@echo "Available targets:"
	@echo "  make install    - Install 1gh to ~/.cargo/bin (system-wide)"
	@echo "  make build      - Build release binary to target/release/1gh"
	@echo "  make run        - Build and run in release mode"
	@echo "  make clean      - Remove build artifacts"
	@echo "  make uninstall  - Remove installed binary"
	@echo "  make sync-version - Sync version from Cargo.toml to docs and packaging"

VERSION := $(shell grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')

sync-version:
	@echo "Syncing version $(VERSION) from Cargo.toml"
	sed -i '' 's|<span>v[^<]*</span>|<span>v$(VERSION)</span>|' docs/index.html
	sed -i '' 's|<string>[0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*</string>|<string>$(VERSION)</string>|' packaging/macos/Info.plist

install:
	cargo install --path .

build: sync-version
	cargo build --release

run:
	cargo run --release

clean:
	cargo clean

uninstall:
	cargo uninstall 1gh
