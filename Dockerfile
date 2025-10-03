FROM rust:1.72-bullseye AS planner
WORKDIR /app

# copy manifest
COPY Cargo.toml Cargo.lock ./

# Install cargo-chef to prepare recipe
RUN cargo install cargo-chef --locked
RUN cargo chef prepare --recipe-path recipe.json

FROM debian:bookworm-slim AS chef
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    ca-certificates \
    build-essential \
    pkg-config \
    libssl-dev \
    git \
    curl \
    wget \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Install rustup and toolchain
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default stable && rustup target add wasm32-unknown-unknown || true

# Install dioxus-cli
RUN cargo install --locked dioxus-cli || true

# copy the prepared recipe
COPY --from=planner /app/recipe.json ./recipe.json

# Install cargo-chef and run cook to precompile dependencies
RUN cargo install cargo-chef --locked
RUN cargo chef cook --release --recipe-path recipe.json

FROM debian:bookworm-slim AS build
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    ca-certificates \
    build-essential \
    pkg-config \
    libssl-dev \
    git \
    curl \
    wget \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Install rustup and toolchain
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default stable && rustup target add wasm32-unknown-unknown || true

# Copy precompiled deps from chef stage
COPY --from=chef /usr/local/cargo/registry /usr/local/cargo/registry
COPY --from=chef /usr/local/cargo/git /usr/local/cargo/git
COPY --from=chef /root/.cargo/bin /root/.cargo/bin

# Copy source
COPY . .

# Build frontend via dioxus-cli
RUN if command -v dx >/dev/null 2>&1; then dx build --release; else echo "dx not available"; fi

# Build release binary
RUN cargo build --release

# Reduce binary size (strip symbols)
RUN strip /app/target/release/qr-craft || true

FROM gcr.io/distroless/cc-debian11
COPY --from=build /app/target/release/qr-craft /usr/local/bin/qr-craft
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/qr-craft"]
