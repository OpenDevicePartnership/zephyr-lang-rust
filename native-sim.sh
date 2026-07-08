#!/bin/bash
# Build + run the zephyr-odp sample on native_sim (64-bit).

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="${BUILD_DIR:-$(west topdir)/build-native}"

# Kill any native_sim instances left running from previous runs.
pkill -f "$BUILD_DIR/zephyr/zephyr.exe" || true

# native_sim/native/64 on a 64-bit host maps to the aarch64/x86_64 bare-metal
# Rust target (see CMakeLists.txt). Pick the one matching the host CPU.
case "$(uname -m)" in
    aarch64|arm64) RUST_TARGET="aarch64-unknown-none" ;;
    x86_64|amd64)  RUST_TARGET="x86_64-unknown-none" ;;
    *) echo "Unsupported host architecture: $(uname -m)" >&2; exit 1 ;;
esac
if command -v rustup >/dev/null 2>&1; then
    rustup target add "$RUST_TARGET"
fi

# Build for native_sim (pass -p always via WEST_BUILD_ARGS for a pristine build).
west build -b native_sim/native/64 "$SCRIPT_DIR/samples/zephyr-odp" -d "$BUILD_DIR" ${WEST_BUILD_ARGS}

# Launch the native binary.
west build -t run -d "$BUILD_DIR"
