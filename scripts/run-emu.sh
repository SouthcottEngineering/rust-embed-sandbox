#!/usr/bin/env bash
set -euo pipefail

TARGET="${TARGET:-aarch64-unknown-linux-gnu}"
BIN="${BIN:-my-rust-pi-app}"
MODE="${MODE:-release}"

echo "Building for target: $TARGET in $MODE mode"
cargo build --target "$TARGET" --"$MODE"

# QEMU registers via binfmt; simply exec the foreign binary.
export RUST_LOG=info
export APP_ENV=test-emu

echo "Running emulated binary with healthcheck"
"target/${TARGET}/${MODE}/${BIN}" --healthcheck