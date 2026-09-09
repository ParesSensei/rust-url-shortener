FROM rust:1-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY .sqlx .sqlx
COPY src src
COPY migrations migrations
COPY static static

ENV SQLX_OFFLINE=true

RUN cargo build --release


FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update \
    && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/url-shortener /app/url-shortener
COPY --from=builder /app/static /app/static

EXPOSE 8080

CMD ["./url-shortener"]
