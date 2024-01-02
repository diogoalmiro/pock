FROM rust:1.75.0-slim-bookworm

WORKDIR /usr/app/pock-server

RUN apt update && apt install -y \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features postgres

RUN mkdir src && echo "fn main() {}" > src/main.rs
COPY Cargo.toml Cargo.lock ./
RUN cargo build

