# Multi-stage Dockerfile for razorpay-rs development, testing, and CI
FROM rust:1.80-slim-bullseye AS builder

WORKDIR /app

# Install build dependencies and OpenSSL tools
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    git \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install rustfmt and clippy
RUN rustup component add rustfmt clippy

# Copy dependency manifests first for layer caching
COPY Cargo.toml Cargo.lock ./
COPY razorpay/Cargo.toml ./razorpay/
COPY tests/Cargo.toml ./tests/
COPY examples/Cargo.toml ./examples/

# Create stub source files to cache dependencies
RUN mkdir -p razorpay/src tests/src tests/tests examples/src/bin && \
    echo "pub fn dummy() {}" > razorpay/src/lib.rs && \
    echo "fn main() {}" > examples/src/bin/quickstart.rs && \
    echo "fn main() {}" > examples/src/bin/create_order.rs && \
    echo "pub fn dummy() {}" > tests/src/lib.rs && \
    cargo build --workspace || true

# Copy full source tree
COPY . .

# Run format verification, clippy linting, and all offline unit tests
RUN cargo fmt --all --check
RUN cargo clippy --workspace --all-targets --all-features -- -D warnings
RUN cargo test --workspace

CMD ["cargo", "test", "--workspace"]
