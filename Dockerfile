FROM rust:1.40 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path . --verbose --features docker

FROM debian:buster-slim
RUN apt-get update && apt-get install -y openssl
COPY --from=builder /usr/local/cargo/bin/danmaku-server /usr/local/bin/danmaku-server
EXPOSE 8080
CMD ["danmaku-server"]
