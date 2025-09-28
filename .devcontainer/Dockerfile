# .devcontainer/Dockerfile
FROM debian:bookworm-slim

# Basics + build toolchain + aarch64 cross + qemu emulation
RUN apt-get update && apt-get install -y --no-install-recommends \
      ca-certificates curl build-essential pkg-config \
      gcc-aarch64-linux-gnu qemu-user-static binfmt-support \
    && rm -rf /var/lib/apt/lists/*

# Non-root user
RUN groupadd -g 1001 ci && useradd -m -u 1001 -g 1001 ci

# User-scoped rustup/cargo homes
ENV HOME=/home/ci \
    CARGO_HOME=/home/ci/.cargo \
    RUSTUP_HOME=/home/ci/.rustup \
    PATH=/home/ci/.cargo/bin:$PATH

USER ci
WORKDIR /work

# Install Rust 1.86 + components in user scope and add aarch64 std
RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.86.0 --profile minimal \
 && rustup component add rustfmt clippy llvm-tools-preview \
 && rustup target add aarch64-unknown-linux-gnu