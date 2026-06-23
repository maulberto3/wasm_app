# Leptos Full-Stack WASM App

A full-stack web application with:
- **Frontend**: Leptos (reactive Rust framework with SSR)
- **Backend**: Axum web framework
- **Optimization**: CMA-ES algorithm via `haru_cmaes` crate
- **Deployment**: Azure (Static Web Apps or App Service)

## Features

✨ **Full-Stack Rust**: Write frontend and backend in Rust  
🎯 **CMA-ES Optimization**: Production-grade Covariance Matrix Adaptation  
⚡ **Server-Side Rendering**: Faster initial page loads  
🔄 **Reactive UI**: Leptos' signals and fine-grained reactivity  
📦 **WASM**: Runs on browser and server  
🚀 **Fast**: Optimized Rust performance  

## Prerequisites

- Rust 1.75+ (stable)
- `cargo-leptos`: `cargo install cargo-leptos`
- `wasm32-unknown-unknown` target

## Quick Start

### Development with Hot Reload

```bash
cargo leptos watch
```

Visit `http://localhost:3000` (server) and `http://localhost:3001` (live reload).

### Production Build

```bash
cargo leptos build --release
./target/release/wasm_app_server
```

Then visit `http://localhost:3000`

## Project Structure

```
src/
├── main.rs              # Axum server (SSR)
├── lib.rs               # Shared code + hydration
├── app.rs               # Leptos app component
└── optimization/
    └── cmaes.rs         # CMA-ES integration

Cargo.toml              # Dependencies
Cargo.toml.leptos       # Leptos config
rust-toolchain.toml     # Explicit Rust version
```

## API Routes

### `POST /api/optimize`

Run CMA-ES optimization.

**Request Body**:
```json
{
  "dimensions": 10,
  "population_size": 15,
  "max_generations": 100
}
```

**Response**:
```json
{
  "best_fitness": 0.0001,
  "best_individual": [0.01, -0.005, ...],
  "generations": 87,
  "elapsed_secs": 2.34
}
```

## Testing

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test
```

## Deployment to Azure

### Option 1: Static Web Apps (Client-Side Only)

```bash
cargo leptos build --release
# Deploy ./target/release/wasm_app_server/site to Static Web Apps
```

### Option 2: App Service (Full-Stack)

```bash
cargo leptos build --release

# Create App Service
az appservice plan create --name wasm-app-plan --resource-group mygroup --sku B1 --is-linux
az webapp create --resource-group mygroup --plan wasm-app-plan --name wasm-app --runtime "RUST|1.75"

# Deploy
./target/release/wasm_app_server
```

## Performance Optimization

- **LTO**: Link-time optimization enabled in release profile
- **Diagonal CMA-ES**: For faster convergence on high-dimensional problems
- **Async Axum**: Non-blocking request handling
- **Fine-grained Reactivity**: Leptos only re-renders changed components

## Resources

- [Leptos Book](https://book.leptos.dev/)
- [haru_cmaes GitHub](https://github.com/kamada-2024/haru_cmaes/)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
- [Azure App Service Rust](https://learn.microsoft.com/azure/app-service/quickstart-rust)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

MIT License - see LICENSE file for details

## References

- [wasm-bindgen Book](https://rustwasm.org/docs/wasm-bindgen/)
- [wasm-pack Documentation](https://rustwasm.org/docs/wasm-pack/)
- [Basin Crate](https://jolars.co/blog/2026-06-10-basin/) - Inspiration
