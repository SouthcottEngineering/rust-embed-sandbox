#!/usr/bin/env bash
set -euo pipefail

TARGET="${TARGET:-aarch64-unknown-linux-gnu}"

echo "Running emulated tests for target: $TARGET"

# Build test binary for target - Note: cross might not be available in CI, so we'll use cargo directly
echo "Building tests for $TARGET..."
cargo build --target "$TARGET" --tests

echo "Running tests with cargo nextest..."
# Run the tests - they should execute under QEMU emulation if configured
cargo nextest run --profile ci --target "$TARGET" || {
    echo "nextest not available, falling back to cargo test"
    cargo test --target "$TARGET"
}