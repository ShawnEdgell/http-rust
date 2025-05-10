FROM rust:1.86-alpine AS builder

RUN apk add --no-cache build-base

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {println!(\"Dummy build for dep caching\")}" > src/main.rs && \
    cargo build --release && \
    rm -rf src target

COPY ./src ./src

RUN cargo build --release

FROM alpine:latest

RUN apk --no-cache add ca-certificates

WORKDIR /app

COPY --from=builder /usr/src/app/target/release/http-rust .

EXPOSE 3000

CMD ["./http-rust"]