################################################################################
# Stage 1 — chef plan (cache crate dependency compilation)
################################################################################
FROM rust:1.95-slim-bookworm AS chef
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev build-essential ca-certificates \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef --version 0.1.68 --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

################################################################################
# Stage 2 — build dependencies (cached layer) + the app + the WASM bundle
################################################################################
FROM chef AS builder
# Tooling needed by cargo-leptos: WASM target, wasm-bindgen, cargo-leptos itself.
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos --version 0.2.42 --locked

# Cook dependencies first so source changes don't bust the cache.
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Now copy the source and build the production bundle.
COPY . .
RUN cargo leptos build --release

################################################################################
# Stage 3 — slim runtime
################################################################################
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
# Compiled server binary.
COPY --from=builder /app/target/release/presentation ./presentation
# Frontend bundle + static assets that cargo-leptos produces.
COPY --from=builder /app/target/site ./site
# Embedded migrations are baked into the binary; nothing to copy.

ENV LEPTOS_SITE_ROOT=/app/site \
    LEPTOS_SITE_PKG_DIR=pkg \
    LEPTOS_OUTPUT_NAME=portfolio \
    LEPTOS_SITE_ADDR=0.0.0.0:8080 \
    DATABASE_URL=sqlite:///data/portfolio.db?mode=rwc \
    RUST_LOG=info

EXPOSE 8080
CMD ["/app/presentation"]
