# Development

FROM rust:latest as develop
WORKDIR /app

RUN cargo install sea-orm-cli cargo-watch

COPY . .
