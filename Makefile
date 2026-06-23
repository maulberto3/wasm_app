.PHONY: help install build build-release run run-release dev test check check-ssr check-hydrate clean fmt docs ci

# ============================================================================
# HELP
# ============================================================================

help:
	@echo "WASM App (Leptos + Axum + haru_cmaes)"
	@echo ""
	@echo "┌─ SETUP ─────────────────────────────────────────────────────────┐"
	@echo "│  make install       Install dependencies (rustup, wasm target)  │"
	@echo "└─────────────────────────────────────────────────────────────────┘"
	@echo ""
	@echo "┌─ DEVELOPMENT ───────────────────────────────────────────────────┐"
	@echo "│  make dev           🚀 Start dev server with hot reload (MAIN)  │"
	@echo "│  make build         Build debug version (client & server)       │"
	@echo "│  make run           Run debug server (after build)              │"
	@echo "└─────────────────────────────────────────────────────────────────┘"
	@echo ""
	@echo "┌─ PRODUCTION ────────────────────────────────────────────────────┐"
	@echo "│  make build-release Build optimized release                     │"
	@echo "│  make run-release   Run release server (after build-release)    │"
	@echo "└─────────────────────────────────────────────────────────────────┘"
	@echo ""
	@echo "┌─ QUALITY & TESTING ─────────────────────────────────────────────┐"
	@echo "│  make check         Run clippy linter                           │"
	@echo "│  make check-ssr     Check SSR (server-side) compilation         │"
	@echo "│  make check-hydrate Check hydrate (client-side WASM)            │"
	@echo "│  make test          Run tests                                   │"
	@echo "└─────────────────────────────────────────────────────────────────┘"
	@echo ""
	@echo "┌─ MAINTENANCE ───────────────────────────────────────────────────┐"
	@echo "│  make fmt           Format Rust code                            │"
	@echo "│  make clean         Clean all build artifacts                   │"
	@echo "│  make docs          Generate & open documentation               │"
	@echo "│  make ci            Run full CI pipeline                        │"
	@echo "└─────────────────────────────────────────────────────────────────┘"

# ============================================================================
# SETUP
# ============================================================================

install:
	@echo "✨ Setting up development environment..."
	@echo "  • Updating Rust to latest stable..."
	rustup update stable
	@echo "  • Adding WASM target..."
	rustup target add wasm32-unknown-unknown
	@echo "  • Installing cargo-leptos..."
	cargo install cargo-leptos
	@echo "✅ Installation complete!"

# ============================================================================
# DEVELOPMENT
# ============================================================================

dev: check-ssr check-hydrate
	@echo "🚀 Starting development server with hot reload..."
	@echo "   Visit http://localhost:3000"
	cargo leptos watch

build:
	@echo "🔨 Building debug version (client + server)..."
	cargo leptos build
	@echo "✅ Build complete!"
	@echo "   Run with: make run"

run: build
	@echo "▶️  Running debug server..."
	./target/debug/wasm_app

# ============================================================================
# PRODUCTION
# ============================================================================

build-release:
	@echo "🔨 Building optimized release (client + server)..."
	cargo leptos build --release
	@echo "✅ Build complete!"
	@echo "   Run with: make run-release"

run-release: build-release
	@echo "▶️  Running release server..."
	./target/release/wasm_app

# ============================================================================
# QUALITY & TESTING
# ============================================================================

check:
	@echo "🔍 Checking code with Clippy..."
	cargo clippy --all-targets --all-features

check-ssr:
	@echo "🔍 Checking SSR (server-side) compilation..."
	cargo check --features ssr

check-hydrate:
	@echo "🔍 Checking hydrate (client-side WASM) compilation..."
	cargo check --lib --no-default-features --features hydrate

test:
	@echo "🧪 Running tests..."
	cargo test --lib --verbose
	@echo "✅ Tests complete!"

# ============================================================================
# MAINTENANCE
# ============================================================================

fmt:
	@echo "🎨 Formatting Rust code..."
	cargo fmt --all
	@echo "✅ Formatting complete!"

clean:
	@echo "🗑️  Cleaning build artifacts..."
	cargo leptos clean
	cargo clean
	rm -rf target/ pkg/
	@echo "✅ Clean complete!"

docs:
	@echo "📖 Generating documentation..."
	cargo doc --no-deps --open --features ssr

# ============================================================================
# CI PIPELINE
# ============================================================================

ci: fmt check test build-release
	@echo "✅ CI pipeline complete!"

