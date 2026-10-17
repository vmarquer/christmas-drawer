
# Étape 1 : compiler l'application Rust
FROM rust:1-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/christmas-drawer /usr/local/bin/christmas-drawer

ENTRYPOINT ["/usr/local/bin/christmas-drawer"]