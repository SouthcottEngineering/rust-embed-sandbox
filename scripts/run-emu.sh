#!/usr/bin/env bash
set -euo pipefail

TARGET="${TARGET:-aarch64-unknown-linux-gnu}"
MODE="${MODE:-release}"
BIN="target/${TARGET}/${MODE}/my-rust-pi-app"

echo "Building for target: $TARGET in $MODE mode"
cargo build --target "$TARGET" --"$MODE"

echo "Running emulated binary with healthcheck"

# Prefer dynamic qemu (works in most containers)
if command -v qemu-aarch64 >/dev/null 2>&1; then
  # -L points QEMU at the AArch64 sysroot so the dynamic linker & libs are found
  exec qemu-aarch64 -L /usr/aarch64-linux-gnu "$BIN" --healthcheck
fi

# Fallback to static qemu if present
if command -v qemu-aarch64-static >/dev/null 2>&1; then
  exec qemu-aarch64-static -L /usr/aarch64-linux-gnu "$BIN" --healthcheck
fi

# Last resort: try direct exec (requires binfmt on the host)
exec "$BIN" --healthcheck