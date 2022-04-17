FROM rust:1.60 as builder
RUN rustup component add rustfmt
WORKDIR /discord_pipe/
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /discord_pipe/target/release/rpc_server .
CMD ["./rpc_server"]