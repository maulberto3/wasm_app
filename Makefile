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
# QUALITY & TESTING
# ============================================================================

check:
	@echo "🔍 Checking code with Clippy..."
	cargo clippy --all-targets --all-features
	@echo "✅ Clippy check complete!"

check-ssr:
	@echo "🔍 Checking SSR (server-side) compilation..."
	cargo check --features ssr 2>&1 | grep -E "error|warning: unused|Finished|Checking"
	@echo "✅ SSR check complete!"

check-hydrate:
	@echo "🔍 Checking hydrate (client-side WASM) compilation..."
	cargo check --lib --no-default-features --features hydrate 2>&1 | grep -E "error|warning: unused|Finished|Checking"
	@echo "✅ Hydrate check complete!"

test:
	@echo "🧪 Running tests..."
	cargo test --lib --verbose
	@echo "✅ Tests complete!"

# ============================================================================
# CI PIPELINE
# ============================================================================

ci: fmt check check-ssr check-hydrate test
	@echo "✅ CI pipeline complete!"

# ============================================================================
# DEVELOPMENT
# ============================================================================

dev: ci
	@echo "🚀 Starting development server with hot reload..."
	@echo "   Visit http://localhost:3000"
	cargo leptos watch

dev-build:
	@echo "🔨 Building debug version (client + server)..."
	cargo leptos build
	@echo "✅ Build complete!"
	@echo "   Run with: make dev-run"
	@echo ""
	@echo "📊 Debug Artifact Sizes:"
	@du -sh target/debug/wasm_app | awk '{print "  Server Binary: " $$1}'
	@du -sh target/site/pkg/ | awk '{print "  WASM Package: " $$1}'

dev-run: dev-build
	@echo "▶️  Running debug server..."
	./target/debug/wasm_app

# ============================================================================
# PRODUCTION
# ============================================================================

build:
	@echo "🔨 Building optimized release (client + server)..."
	cargo leptos build --release
	@echo "✅ Build complete!"
	@echo "   Run with: make run"
	@echo ""
	@echo "📊 Release Artifact Sizes:"
	@du -sh target/release/wasm_app | awk '{print "  Server Binary: " $$1}'
	@du -sh target/site/pkg/ | awk '{print "  WASM Package: " $$1}'

run: build
	@echo "▶️  Running release server..."
	./target/release/wasm_app