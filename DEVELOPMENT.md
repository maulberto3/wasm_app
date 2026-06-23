# Development Setup Guide

## Quick Start

```bash
make install
make dev
```

Then visit `http://localhost:3000` in your browser.

The live reload server runs on `http://localhost:3001`.

## File Structure

```
wasm_app/
├── src/
│   ├── main.rs                # Axum server (SSR)
│   ├── lib.rs                 # Shared code + hydration
│   ├── app.rs                 # Leptos app component
│   └── optimization/
│       └── cmaes.rs           # CMA-ES optimization
├── style/
│   └── main.css               # Leptos styles
├── public/                    # Static assets
├── .github/
│   └── workflows/             # CI/CD pipelines
├── Cargo.toml                 # Rust dependencies
├── Cargo.toml.leptos          # Leptos configuration
├── rust-toolchain.toml        # Explicit Rust version
└── Makefile                   # Development commands
```

## Available Commands

| Command | Description |
|---------|-------------|
| `make install` | Install cargo-leptos and toolchain |
| `make dev` | Start dev server with hot reload |
| `make build` | Build debug version |
| `make build-release` | Build optimized release |
| `make test` | Run all tests |
| `make clean` | Clean build artifacts |
| `make fmt` | Format Rust code |
| `make check` | Lint code with Clippy |
| `make run-release` | Run release server |

## Development Workflow

1. **Start development server**:
   ```bash
   make dev
   ```
   This runs:
   - Axum server on http://localhost:3000
   - Live reload on http://localhost:3001
   - Automatic recompilation on file changes

2. **Make changes**:
   - Edit Rust code in `src/`
   - Edit styles in `style/main.css`
   - Changes auto-reload in browser

3. **Run tests locally**:
   ```bash
   cargo test --lib --verbose
   ```

4. **Format and check**:
   ```bash
   make fmt
   make check
   ```

## Architecture

### Full-Stack Rendering

Leptos with SSR mode means:
1. Server renders HTML on first request
2. Browser hydrates and takes over
3. Client can run Leptos components

### Server Functions (Planned)

You can create server functions like:
```rust
#[server]
async fn optimize_on_server(params: OptimizationParams) -> Result<OptimizationResult, ServerFnError> {
    use wasm_app::optimization::optimize_sum_of_squares;
    Ok(optimize_sum_of_squares(params))
}
```

Then call from client:
```rust
let result = optimize_on_server(params).await?;
```

## Setting Up Azure Deployment

### 1. Create Azure App Service

```bash
# Create resource group
az group create --name wasm-app-rg --location eastus

# Create App Service Plan
az appservice plan create \
  --name wasm-app-plan \
  --resource-group wasm-app-rg \
  --sku B1 \
  --is-linux

# Create web app
az webapp create \
  --resource-group wasm-app-rg \
  --plan wasm-app-plan \
  --name wasm-app \
  --runtime "RUST|1.75"
```

### 2. Configure GitHub Secrets

In GitHub repo settings, add:
- `AZURE_APP_NAME`: Your app name (e.g., `wasm-app`)
- `AZURE_PUBLISH_PROFILE`: Download from Azure Portal (App Service > Deployment > Deployment Center)

### 3. Deploy

Push to main branch - GitHub Actions will auto-deploy.

## Local Testing

### Browser Testing

```bash
# Start dev server
make dev

# Open http://localhost:3000 and test the UI
# Open http://localhost:3001 for live reload
```

### Run Tests

```bash
# Unit tests
cargo test --lib

# All tests (includes integration tests)
cargo test

# With output
cargo test -- --nocapture --test-threads=1
```

## Debugging

### View Leptos Logs

Browser console will show:
- Leptos warnings/errors
- Server responses
- Hydration details

### Server Logs

The server prints request logs to stdout when running `make dev`.

### WASM Bundle Size

```bash
# After build
ls -lh target/release/wasm_app_server/site/pkg/*.wasm

# Gzipped size
gzip -c target/release/wasm_app_server/site/pkg/wasm_app*.wasm | wc -c
```

### Profile Code

```bash
# Generate FlameGraph
cargo install flamegraph
cargo flamegraph --bin wasm_app_server -- --help

# View with: firefox flamegraph.svg
```

## Common Issues

### Port Already in Use

```bash
# Kill process on port 3000
lsof -i :3000 | grep LISTEN | awk '{print $2}' | xargs kill -9

# Or use different port
LEPTOS_SITE_ADDR=127.0.0.1:3001 make dev
```

### Hydration Mismatch

Leptos will warn if server HTML doesn't match client. Check:
- Is component output deterministic?
- Are you using client-side randomness or timestamps?

### Build Fails

```bash
# Clean and rebuild
make clean
cargo build --features ssr

# Check for errors
cargo check --all-targets --all-features
```

### Hot Reload Not Working

- Check port 3001 is accessible
- Clear browser cache (Ctrl+Shift+Delete)
- Restart `make dev`

## Performance Optimization

### Build Optimizations

Already enabled in `Cargo.toml`:
- LTO (Link-Time Optimization)
- Minimal code generation units

### Runtime Optimization

1. **Use `create_resource` for async**:
   ```rust
   let data = create_resource(
       || (),
       |_| async { fetch_expensive_data().await },
   );
   ```

2. **Memoize expensive computations**:
   ```rust
   let result = create_memo(move || {
       expensive_computation(value())
   });
   ```

3. **Lazy load components**:
   ```rust
   <Suspense fallback=|| "Loading...">
       <Component/>
   </Suspense>
   ```

## Resources

- [Leptos Book](https://book.leptos.dev/)
- [Leptos API](https://docs.rs/leptos/latest/leptos/)
- [haru_cmaes Docs](https://docs.rs/haru_cmaes/latest/haru_cmaes/)
- [Axum Guide](https://github.com/tokio-rs/axum/tree/main/examples)
- [Rust Async Programming](https://rust-lang.github.io/async-book/)

## Next Steps

1. Modify `src/app.rs` to build your UI
2. Add optimization logic to `src/optimization/cmaes.rs`
3. Create server functions for heavy computations
4. Test locally with `make dev`
5. Deploy to Azure with GitHub Actions

## File Structure

```
wasm_app/
├── src/
│   └── lib.rs                 # Main Rust library
├── tests/
│   └── integration_tests.rs   # Integration tests
├── www/
│   └── index.html             # Web demo
├── .github/
│   └── workflows/             # CI/CD pipelines
├── Cargo.toml                 # Rust dependencies
├── package.json               # Node.js metadata
├── Makefile                   # Development commands
└── staticwebapp.config.json   # Azure configuration
```

## Available Commands

| Command | Description |
|---------|-------------|
| `make install` | Install Rust WASM toolchain |
| `make build` | Build WASM (debug) |
| `make build-release` | Build WASM (optimized) |
| `make test` | Run Rust unit tests |
| `make test-wasm` | Run WASM integration tests |
| `make clean` | Clean build artifacts |
| `make dev-server` | Start local development server |
| `make fmt` | Format Rust code |
| `make check` | Lint code with Clippy |
| `make docs` | Generate and open documentation |

## Setting Up Azure Deployment

### 1. Create Azure Static Web App

```bash
# In Azure Portal, create a new Static Web App resource
# - Name: wasm-app
# - Region: Choose nearest to you
- Deployment details: GitHub
- Select your repository
- Build presets: Custom
- Build location: dist
- Build command: wasm-pack build --release --target web --out-dir dist/pkg
```

### 2. Configure GitHub Secret

1. Go to your repository **Settings** → **Secrets and variables** → **Actions**
2. Create new secret: `AZURE_STATIC_WEB_APPS_API_TOKEN`
3. Copy deployment token from Azure Portal

### 3. Update `.github/workflows/deploy-azure.yml`

Update the `app_location` and `output_location` to match your build setup.

## Setting Up npm Publishing

### 1. Create npm Account

- Sign up at [npmjs.com](https://npmjs.com)
- Create an access token

### 2. Add GitHub Secret

1. Go to repository **Settings** → **Secrets and variables** → **Actions**
2. Create secret: `NPM_TOKEN`
3. Paste your npm access token

### 3. Publish Release

```bash
# Tag and push release
git tag v0.1.0
git push origin v0.1.0

# GitHub Actions will automatically publish to npm
```

## Local Testing

### Browser Testing

```bash
# Build and serve locally
make dev-server

# Open http://localhost:8000 and test the demo
```

### WASM Integration Tests

```bash
# Requires Firefox
wasm-pack test --headless --firefox tests/integration_tests.rs
```

## Debugging

### View WASM Module Size

```bash
ls -lh pkg/*.wasm
gzip -c pkg/*.wasm | wc -c  # Gzipped size
```

### Profile in Browser

1. Open DevTools (F12)
2. Go to **Performance** tab
3. Record interactions with the WASM app
4. Analyze execution time

### View Rust Backtrace

```bash
# Enable debug info when building
RUST_BACKTRACE=1 make build-release
```

## Common Issues

### WASM Module Not Loading

- Check browser console for errors
- Verify MIME types in `staticwebapp.config.json`
- Ensure `.wasm` file is properly served

### Bundle Size Too Large

- Enable LTO in `Cargo.toml`:
  ```toml
  [profile.release]
  lto = true
  ```
- Use `wasm-opt` post-build optimization
- Remove unused dependencies

### Tests Failing

```bash
# Run tests with verbose output
cargo test --lib -- --nocapture

# Check test output
cargo test --lib -- --test-threads=1
```

## Resources

- [wasm-bindgen Book](https://rustwasm.org/docs/wasm-bindgen/)
- [wasm-pack Guide](https://rustwasm.org/docs/wasm-pack/)
- [Rust WASM Best Practices](https://rustwasm.org/docs/wasm-bindgen/reference/optimization.html)
- [Azure Static Web Apps](https://learn.microsoft.com/azure/static-web-apps/)

## Next Steps

1. Write your core Rust logic in `src/lib.rs`
2. Add tests in `src/lib.rs` and `tests/integration_tests.rs`
3. Build and test locally with `make build && make test`
4. Create a GitHub repository and push your code
5. Configure Azure deployment
6. Push to `main` to trigger deployment

Happy coding! 🚀
