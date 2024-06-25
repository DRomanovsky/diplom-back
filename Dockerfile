## Build stage from rust
FROM rust:1.78.0-buster as builder

WORKDIR /app

## accept the build arg
ARG DATABASE_URL
ARG JWT_SECRET
ARG HASH_SECRET

ENV DATABASE_URL=$DATABASE_URL
ENV JWT_SECRET=$JWT_SECRET
ENV HASH_SECRET=$HASH_SECRET

RUN apt update
RUN apt install -y libpq-dev libclang-dev clang

RUN cargo install diesel_cli --no-default-features --features postgres

COPY . /app/

COPY .env /app/.env

RUN cargo build --release

EXPOSE 8080

ENTRYPOINT ["/bin/bash", "-c", "cargo run --release"]
