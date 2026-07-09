#!/bin/bash
# Build and automatically open the Zephyr Rust documentation.
# On WSL it opens the docs in the Windows default browser; on native Linux
# it opens them with xdg-open.
#
# Run from anywhere inside the west workspace; the build dir is resolved
# relative to the workspace top level (override with the BUILD_DIR env var).

set -e

# Detect whether we're running under WSL or native Linux.
if [ -n "$WSL_DISTRO_NAME" ] || grep -qiE "(microsoft|wsl)" /proc/version 2>/dev/null; then
    IS_WSL=1
    echo "Detected WSL."
else
    IS_WSL=0
    echo "Detected Native Linux."
fi

WS="$(west topdir)"
BUILD_DIR="${BUILD_DIR:-$WS/build}"

# 1. Run the west build target
west build -d "$BUILD_DIR" -t rustdoc

# 2. Find the generated index.html file dynamically
LINUX_PATH="$(find "$BUILD_DIR" -name "index.html" | grep "doc/rustapp/index.html" | head -n 1)"

if [ -n "$LINUX_PATH" ]; then
    if [ "$IS_WSL" -eq 1 ]; then
        # Convert the Linux path to a Windows path and launch the default browser
        WIN_PATH="$(wslpath -w "$LINUX_PATH")"
        cmd.exe /c start "" "$WIN_PATH"
    else
        # Open with the default Linux browser handler
        xdg-open "$LINUX_PATH"
    fi
else
    echo "Error: Could not locate the generated index.html file."
    exit 1
fi
