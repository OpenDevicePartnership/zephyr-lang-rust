#!/bin/bash
# Build + run the zephyr-odp sample on native_sim (64-bit).

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="${BUILD_DIR:-$(west topdir)/build-native}"

# native_sim/native/64 on a 64-bit host maps to the aarch64/x86_64 bare-metal
# Rust target
if command -v rustup >/dev/null 2>&1; then
    rustup target add aarch64-unknown-none
fi

# Build for native_sim (pass -p always via WEST_BUILD_ARGS for a pristine build).
west build -b native_sim/native/64 "$SCRIPT_DIR/samples/zephyr-odp" -d "$BUILD_DIR" ${WEST_BUILD_ARGS}

# Launch the native binary.
west build -t run -d "$BUILD_DIR"
