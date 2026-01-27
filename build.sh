#!/bin/bash
# WhisperTray build script

cd "$(dirname "$0")"

# Set library path for libxdo
export LIBRARY_PATH="/usr/lib/x86_64-linux-gnu:$LIBRARY_PATH"

# Build mode (debug or release)
MODE="${1:-debug}"

if [ "$MODE" = "release" ]; then
    echo "Building release version..."
    cargo build --manifest-path src-tauri/Cargo.toml --release
    echo "Binary: src-tauri/target/release/whispertray"
else
    echo "Building debug version..."
    cargo build --manifest-path src-tauri/Cargo.toml
    echo "Binary: src-tauri/target/debug/whispertray"
fi
