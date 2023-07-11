# Development

FROM rust:latest
WORKDIR /app

RUN cargo install sea-orm-cli cargo-watch

COPY . .
