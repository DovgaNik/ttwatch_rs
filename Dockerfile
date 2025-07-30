FROM rust:latest as builder

RUN apt update & apt upgrade -y

WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

RUN cargo build --release
RUN rm src/*.rs

# The final base image
FROM debian:latest

RUN apt update & apt upgrade -y

COPY --from=builder /app/target/release/tiktok_video_watcher /usr/src/app

CMD ["/usr/src/app"]
