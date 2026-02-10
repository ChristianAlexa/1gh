.PHONY: install build run clean uninstall help

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

install:
	cargo install --path .

build:
	cargo build --release

run:
	cargo run --release

clean:
	cargo clean

uninstall:
	cargo uninstall 1gh
