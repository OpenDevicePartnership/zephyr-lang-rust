#!/bin/bash
# Build and automatically open the Zephyr Rust documentation for every configured
# build directory found in the workspace.
# Empty or not-yet-configured build directories are skipped.
#
# On WSL it opens the docs in the Windows default browser, on native Linux
# it opens them with xdg-open.
#

set -e

# Detect whether we're running under WSL or native Linux.
if [ -n "$WSL_DISTRO_NAME" ] || grep -qiE "(microsoft|wsl)" /proc/version 2>/dev/null; then
    IS_WSL=1
    echo "Detected WSL."
else
    IS_WSL=0
    echo "Detected Native Linux."
fi

# Open a generated doc file with the platform's default browser handler.
open_doc() {
    local path="$1"
    if [ "$IS_WSL" -eq 1 ]; then
        # Convert the Linux path to a Windows path and launch the default browser.
        cmd.exe /c start "" "$(wslpath -w "$path")"
    else
        # Open with the default Linux browser handler.
        xdg-open "$path"
    fi
}

WS="$(west topdir)"

# Collect the build directories to document. If BUILD_DIR is set, only use that one.
# Otherwise, document $WS/build plus any $WS/build-* variant.
BUILD_DIRS=()
if [ -n "$BUILD_DIR" ]; then
    BUILD_DIRS+=("$BUILD_DIR")
else
    for d in "$WS"/build "$WS"/build-*; do
        [ -d "$d" ] && BUILD_DIRS+=("$d")
    done
fi

if [ "${#BUILD_DIRS[@]}" -eq 0 ]; then
    echo "No build directories found under $WS."
    exit 0
fi

generated_any=0
for dir in "${BUILD_DIRS[@]}"; do
    # Skip anything that isn't a configured Zephyr build (empty / not yet built).
    if [ ! -f "$dir/build.ninja" ] && [ ! -f "$dir/CMakeCache.txt" ]; then
        echo "Skipping $dir (empty or not a configured build directory)."
        continue
    fi

    echo "Generating Rust docs for $dir ..."
    if ! west build -d "$dir" -t rustdoc; then
        echo "Warning: rustdoc build failed for $dir; skipping."
        continue
    fi

    # Find the generated index.html for the rustapp crate in this build dir.
    DOC_PATH="$(find "$dir" -path "*doc/rustapp/index.html" | head -n 1)"
    if [ -n "$DOC_PATH" ]; then
        echo "Opening $DOC_PATH"
        open_doc "$DOC_PATH"
        generated_any=1
    else
        echo "Warning: could not locate the generated index.html in $dir."
    fi
done

if [ "$generated_any" -eq 0 ]; then
    echo "Error: no documentation was generated."
    exit 1
fi
