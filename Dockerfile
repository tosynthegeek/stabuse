FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY . .

COPY Cargo.toml Cargo.lock ./
RUN cargo update
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app

COPY --from=builder /usr/src/app/target/release/stabuse .

EXPOSE 8080

CMD ["./stabuse"]

