# Development

FROM rust:latest as develop
WORKDIR /app

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

COPY . .
