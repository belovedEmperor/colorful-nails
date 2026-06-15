# ── base: system deps + tools ──────────────────────────────────────────
FROM rust:alpine AS base
RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen pkgconf openssl-dev openssl-libs-static
RUN cargo install cargo-chef
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh
WORKDIR /work

# ── planner: scan workspace → recipe.json ─────────────────────────────
FROM base AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ── cacher: compile deps only (cached unless Cargo.lock changes) ───────
FROM base AS cacher
COPY rust-toolchain.toml .
RUN rustup target add wasm32-unknown-unknown
COPY --from=planner /work/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
RUN cargo chef cook --release --target wasm32-unknown-unknown --recipe-path recipe.json

# ── builder: compile app (inherits prebuilt deps from cacher) ──────────
FROM cacher AS builder
COPY . .
ENV LEPTOS_TAILWIND_VERSION=v3.4.17
RUN cargo leptos build --release -vv

# ── runner ─────────────────────────────────────────────────────────────
FROM rust:alpine AS runner
WORKDIR /app
COPY --from=builder /work/target/release/server /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/
EXPOSE $PORT
ENV LEPTOS_SITE_ROOT=./site
CMD ["/app/server"]
