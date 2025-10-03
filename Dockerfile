FROM debian:bookworm-slim AS build
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    ca-certificates \
    build-essential \
    pkg-config \
    libssl-dev \
    git \
    curl \
    ca-certificates \
    wget \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Install rustup and toolchain
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Ensure stable toolchain and wasm target for building web assets
RUN rustup default stable \
    && rustup target add wasm32-unknown-unknown || true

# Install dioxus-cli so we can run `dx build` inside the container
# Install dioxus-cli in its own layer (rarely changes)
RUN cargo install --locked dioxus-cli || true

# Leverage Docker layer cache for Cargo dependencies: copy manifests first
COPY Cargo.toml Cargo.lock ./
# Create dummy src to allow `cargo fetch` of deps without sources
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Fetch and build dependencies (this layer is cached until Cargo.toml changes)
RUN rustup component add rustfmt || true
RUN cargo fetch --locked

# Now copy the real source and build assets
COPY . .

# Run dioxus build to produce web assets (web/dist) if dx available
RUN if command -v dx >/dev/null 2>&1; then dx build --release; else echo "dx not available"; fi

# Build release binary
RUN cargo build --release

# Reduce binary size (strip symbols) - strip is provided by build-essential/binutils
RUN strip /app/target/release/qr-craft || true

# Final runtime: use distroless (glibc-compatible) for a minimal secure image
FROM gcr.io/distroless/cc-debian11
COPY --from=build /app/target/release/qr-craft /usr/local/bin/qr-craft
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/qr-craft"]
