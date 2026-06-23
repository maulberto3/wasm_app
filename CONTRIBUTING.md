# WASM App Contribution Guidelines

## Getting Started

### Prerequisites
- Rust 1.70 or later
- wasm-pack
- Node.js 16+ (optional)

### Initial Setup

```bash
# Install Rust toolchain
make install

# Build the project
make build

# Run tests
make test
```

## Development Workflow

1. **Create a feature branch** from `develop`
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** and test locally
   ```bash
   make test
   make build
   ```

3. **Format and lint your code**
   ```bash
   cargo fmt
   cargo clippy
   ```

4. **Push and create a Pull Request** against the `develop` branch

## Code Style

- Follow Rust naming conventions
- Use `cargo fmt` for formatting
- Fix all `cargo clippy` warnings
- Add tests for new functionality
- Keep bundle size in mind for WASM

## Testing

```bash
# Run unit tests
cargo test --lib

# Run WASM integration tests (requires wasm-pack)
wasm-pack test --headless --firefox

# Run with coverage
cargo tarpaulin
```

## Performance Considerations

- Monitor WASM bundle size: `ls -lh pkg/*.wasm`
- Profile with browser DevTools
- Consider using `wasm-opt` for further optimization

## Commits

Use conventional commit messages:
- `feat: Add new feature`
- `fix: Fix bug in component`
- `docs: Update documentation`
- `test: Add test cases`
- `perf: Improve performance`
- `refactor: Refactor code structure`

## Pull Request Process

1. Update documentation if needed
2. Add tests for new functionality
3. Ensure all CI checks pass
4. Request review from @codeowners
5. Squash commits before merging to `main`

## Reporting Issues

Please include:
- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, browser, WASM version)

## Questions?

Open an issue or discussion in the repository!
