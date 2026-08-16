# Multi-stage Dockerfile for razorpay-rs development, testing, and CI
FROM rust:slim-bookworm AS builder

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

# Copy full source tree
COPY . .

# Run format verification, clippy linting, and all offline unit tests
RUN cargo fmt --all --check
RUN cargo clippy --workspace --all-targets --all-features -- -D warnings
RUN cargo test --workspace

CMD ["cargo", "test", "--workspace"]
