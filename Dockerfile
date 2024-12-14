FROM rust:latest AS builder 

ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=musl-gcc
ENV RUSTFLAGS="-C target-feature=-crt-static"

WORKDIR /usr/src/app 

RUN apt-get update && apt-get install -y \ 
    build-essential \ 
    pkg-config \ 
    libssl-dev \ 
    ca-certificates \
    musl-tools  # Add musl tools for cross-compilation

RUN rustup target add x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./ 
RUN mkdir src && echo "fn main() {}" > src/main.rs 

RUN cargo build --release --target x86_64-unknown-linux-musl
RUN rm -rf src 

COPY . . 
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM frolvlad/alpine-glibc:latest 

WORKDIR /app 

RUN apk add --no-cache \ 
    ca-certificates 

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/stabuse . 

EXPOSE 8080 
CMD ["./stabuse"]