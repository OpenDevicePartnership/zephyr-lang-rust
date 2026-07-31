#!/bin/bash
# Builds and runs the QEMU build.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="${BUILD_DIR:-$(west topdir)/build-qemu}"

if command -v rustup >/dev/null 2>&1; then
    rustup target add thumbv7m-none-eabi
fi

# Build for QEMU (pass -p always via WEST_BUILD_ARGS for a pristine build).
west build -b qemu_cortex_m3 "$SCRIPT_DIR/samples/zephyr-odp" -d "$BUILD_DIR" ${WEST_BUILD_ARGS}

# Launch the image in QEMU.
west build -t run -d "$BUILD_DIR"
