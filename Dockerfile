FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner

COPY --from=builder /usr/local/cargo/bin/rust-rocket-sample /usr/local/bin/rust-rocket-sample
COPY --from=builder /app/.env .env
COPY --from=builder /app/Rocket.toml Rocket.toml

EXPOSE 8080

CMD ["rust-rocket-sample"]