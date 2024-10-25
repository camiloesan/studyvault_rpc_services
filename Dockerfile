FROM rust:latest AS builder

EXPOSE 50051

RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev openssl libssl-dev

WORKDIR /usr/src/sv_rpc_server

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --release

FROM ubuntu:22.04

COPY --from=builder /usr/src/sv_rpc_server/target/release/studyvault-server /usr/local/bin/studyvault-server

CMD ["studyvault-server"]
