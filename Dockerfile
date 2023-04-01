FROM rust:1.61.0-slim as builder

RUN apt-get update && apt-get install -y musl-tools protobuf-compiler

WORKDIR /usr/src

RUN USER=root cargo new medium-rust-dockerize

WORKDIR /usr/src/medium-rust-dockerize


COPY Cargo.toml Cargo.lock ./

RUN rustup target add x86_64-unknown-linux-musl

RUN cargo build --target x86_64-unknown-linux-musl --release


COPY build.rs ./
COPY proto ./proto/

RUN touch /usr/src/medium-rust-dockerize/src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release


RUN rm -rf src
COPY src ./src/

RUN touch /usr/src/medium-rust-dockerize/src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release

COPY migrations ./migrations



FROM alpine:3.16.0 AS runtime

COPY --from=builder /usr/src/medium-rust-dockerize/target/x86_64-unknown-linux-musl/release /usr/local/bin

CMD ["/usr/local/bin/recoining-api-core"]
