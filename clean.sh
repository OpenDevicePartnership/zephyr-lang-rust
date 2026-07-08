#!/bin/bash
# Remove every Zephyr/west build directory for this project so the next build is pristine.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Find the west workspace top level.
if command -v west >/dev/null 2>&1; then
    WS="$(west topdir)"
else
    WS="$(cd "$SCRIPT_DIR/../../.." && pwd)"
fi

BUILD_DIRS=(
    "$SCRIPT_DIR"/build
    "$SCRIPT_DIR"/docgen/build
)
for d in "$WS"/build "$WS"/build-*; do
    [ -d "$d" ] && BUILD_DIRS+=("$d")
done

removed_any=0
for d in "${BUILD_DIRS[@]}"; do
    if [ -d "$d" ]; then
        echo "Removing $d"
        rm -rf "$d"
        removed_any=1
    fi
done

if [ "$removed_any" -eq 0 ]; then
    echo "Nothing to clean; no build directories found."
else
    echo "Clean complete."
fi
