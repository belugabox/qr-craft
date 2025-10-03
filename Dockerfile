FROM debian:bookworm-slim AS build
RUN apt-get update && apt-get install -y ca-certificates build-essential pkg-config libssl-dev git && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Install rustup and toolchain
RUN apt-get update && apt-get install -y curl && rm -rf /var/lib/apt/lists/*
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY . .
RUN rustup default stable && \
    cargo build --release

FROM debian:bookworm-slim
COPY --from=build /app/target/release/qr-craft /usr/local/bin/qr-craft
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/qr-craft"]
