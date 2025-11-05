# Hexendrum Music Player Makefile

.PHONY: help build run test clean release install uninstall setup-dev format lint check

# Default target
help:
	@echo "🎵 Hexendrum Music Player - Available targets:"
	@echo ""
	@echo "📦 Build targets:"
	@echo "  build      - Build the project in debug mode"
	@echo "  release    - Build the project in release mode"
	@echo "  clean      - Clean build artifacts"
	@echo ""
	@echo "🚀 Run targets:"
	@echo "  run            - Run the backend in debug mode"
	@echo "  run-release    - Run the backend in release mode"
	@echo "  run-full       - Run both frontend and backend together (dev mode)"
	@echo "  run-full-release - Run both frontend and backend (release backend, dev frontend)"
	@echo ""
	@echo "🧪 Development targets:"
	@echo "  test       - Run tests"
	@echo "  check      - Check code without building"
	@echo "  format     - Format code with rustfmt"
	@echo "  lint       - Lint code with clippy"
	@echo "  setup-dev  - Setup development environment"
	@echo ""
	@echo "📁 Install targets:"
	@echo "  install    - Install the application"
	@echo "  uninstall  - Uninstall the application"

# Build targets
build:
	@echo "🔨 Building Hexendrum in debug mode..."
	cargo build

release:
	@echo "🚀 Building Hexendrum in release mode..."
	cargo build --release

clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean

# Run targets
run: build
	@echo "🎵 Running Hexendrum (backend only)..."
	cargo run

run-release: release
	@echo "🎵 Running Hexendrum (backend only, release mode)..."
	cargo run --release

run-full: build
	@echo "🎵 Running Hexendrum (frontend + backend)..."
	@echo "📦 Starting backend and frontend..."
	npm run dev

run-full-release: release
	@echo "🎵 Running Hexendrum (frontend + backend, release mode)..."
	@echo "⚠️  Note: Frontend still runs in dev mode. Use 'make build-frontend' for production build."
	npm run dev

# Development targets
test:
	@echo "🧪 Running tests..."
	cargo test

check:
	@echo "🔍 Checking code..."
	cargo check

format:
	@echo "✨ Formatting code..."
	cargo fmt

lint:
	@echo "🔍 Linting code..."
	cargo clippy

setup-dev:
	@echo "⚙️  Setting up development environment..."
	@chmod +x scripts/setup-dev.sh
	./scripts/setup-dev.sh

# Install targets
install: release
	@echo "📦 Installing Hexendrum..."
	@sudo cp target/release/hexendrum /usr/local/bin/
	@echo "✅ Installation complete!"

uninstall:
	@echo "🗑️  Uninstalling Hexendrum..."
	@sudo rm -f /usr/local/bin/hexendrum
	@echo "✅ Uninstallation complete!"

# Quick development workflow
dev: format lint check test build

# Full release workflow
dist: clean format lint check test release

# Show project info
info:
	@echo "🎵 Hexendrum Music Player"
	@echo "📁 Project directory: $(PWD)"
	@echo "🔧 Rust version: $(shell cargo --version 2>/dev/null || echo 'Not available')"
	@echo "📦 Cargo version: $(shell cargo --version 2>/dev/null || echo 'Not available')"
	@echo "📊 Project size: $(shell du -sh . 2>/dev/null | cut -f1 || echo 'Unknown')"
