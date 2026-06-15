FROM rust:alpine AS builder

RUN apk update && \
   apk add --no-cache bash curl npm libc-dev binaryen pkgconf openssl-dev

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh

# Add wasm32 target AFTER copying project so rust-toolchain.toml is respected

WORKDIR /work
COPY . .

RUN rustup target add wasm32-unknown-unknown

ENV LEPTOS_TAILWIND_VERSION=v3.4.17

RUN cargo leptos build --release -vv

FROM rust:alpine AS runner

WORKDIR /app

COPY --from=builder /work/target/release/server /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

EXPOSE $PORT
ENV LEPTOS_SITE_ROOT=./site

CMD ["/app/server"]
