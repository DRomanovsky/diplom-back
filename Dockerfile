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

## copy the project files
COPY . .

## install the project dependencies
RUN cargo build --release

## copy the binary to the final stage
FROM debian:buster-slim

RUN apt update
RUN apt install -y libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /usr/local/bin

## copy the binary from the builder stage
COPY --from=builder /app/target/release/diplom-backv2 .

## expose the port that the application listens on
EXPOSE 8080

## run the application
CMD ["./diplom-backv2"]

