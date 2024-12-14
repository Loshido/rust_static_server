FROM rust:latest AS building
WORKDIR /app

COPY . .

RUN rustup target add aarch64-unknown-linux-musl
RUN cargo build --release --target aarch64-unknown-linux-musl

FROM alpine:latest AS production
WORKDIR /app

COPY --from=building \
/app/target/aarch64-unknown-linux-musl/release/static_server /app/listen