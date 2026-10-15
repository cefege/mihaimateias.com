FROM rust:1-bookworm AS builder
WORKDIR /app

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binary.sh | bash \
 && cargo binstall dioxus-cli@0.7.10 -y \
 && rustup target add wasm32-unknown-unknown

COPY . .
RUN dx bundle --web --release

FROM debian:bookworm-slim
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/dx/mihaimateias/release/web/ /usr/local/app/
WORKDIR /usr/local/app
ENV IP=0.0.0.0 PORT=8080
EXPOSE 8080
USER 65532:65532
ENTRYPOINT ["/usr/local/app/server"]
LABEL org.opencontainers.image.source=https://github.com/cefege/mihaimateias.com
