FROM rust:latest as builder

WORKDIR /app

# Install dependencies
RUN cargo install cargo-leptos && \
    rustup target add wasm32-unknown-unknown

COPY . .

# Build application
RUN cargo leptos build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/wasm_app_server /app/

WORKDIR /app

EXPOSE 3000

ENV LEPTOS_SITE_ADDR=0.0.0.0:3000

CMD ["./wasm_app_server"]
