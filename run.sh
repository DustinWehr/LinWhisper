#!/bin/bash
# WhisperTray startup script

cd "$(dirname "$0")"

# Set library path for libxdo
export LIBRARY_PATH="/usr/lib/x86_64-linux-gnu:$LIBRARY_PATH"

# Set Rust log level (optional - uncomment for more verbose logging)
# export RUST_LOG=info

# Run mode
MODE="${1:-dev}"

if [ "$MODE" = "dev" ]; then
    # Development mode: starts frontend dev server + Tauri app
    npm run tauri dev
elif [ "$MODE" = "prod" ]; then
    # Production mode: build frontend first, then run Tauri
    npm run build
    cargo run --manifest-path src-tauri/Cargo.toml --release
else
    # Direct cargo run (frontend must be built already)
    cargo run --manifest-path src-tauri/Cargo.toml "$@"
fi
