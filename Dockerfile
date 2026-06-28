# Build stage
FROM rustlang/rust:nightly-trixie AS builder

# Install cargo-binstall for easier tool installation
RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN cp cargo-binstall /usr/local/cargo/bin

# Install required build tools
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends clang \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Install cargo-leptos
RUN cargo binstall cargo-leptos -y

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

# Set working directory
WORKDIR /app
COPY . .

# Build the app in release mode
RUN cargo leptos build --release -vv

# Display artifact sizes
RUN echo "=== Build Complete - Artifact Sizes ===" && \
    echo "" && \
    echo "Server Binary Size (Release):" && \
    du -sh /app/target/release/wasm_app && \
    echo "" && \
    echo "WASM Package Size:" && \
    du -sh /app/target/site/pkg/ && \
    echo "" && \
    echo "Total Release Build Size:" && \
    du -sh /app/target/release/

# Runtime stage
FROM debian:trixie-slim AS runtime

WORKDIR /app

# Install runtime dependencies
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Copy the server binary from builder
COPY --from=builder /app/target/release/wasm_app /app/

# Copy static assets (WASM, JS, CSS, HTML)
COPY --from=builder /app/target/site /app/site

# Copy Cargo.toml (needed at runtime for config)
COPY --from=builder /app/Cargo.toml /app/

# Set environment variables
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"

# Expose port
EXPOSE 8080

# Run the server
CMD ["/app/wasm_app"]
