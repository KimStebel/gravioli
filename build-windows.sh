#!/bin/bash
set -euo pipefail

TARGET="x86_64-pc-windows-gnu"
OUTPUT_DIR="build/windows"

# Ensure the Rust target is installed
if ! rustup target list --installed | grep -q "$TARGET"; then
    echo "Installing Rust target $TARGET..."
    rustup target add "$TARGET"
fi

# Ensure MinGW cross-compiler is available
if ! command -v x86_64-w64-mingw32-gcc &>/dev/null; then
    echo "Error: x86_64-w64-mingw32-gcc not found."
    echo "Install it with: sudo apt install gcc-mingw-w64-x86-64"
    exit 1
fi

echo "Building for $TARGET..."
cargo build --release --target "$TARGET"

echo "Packaging..."
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"
cp "target/$TARGET/release/gravioli.exe" "$OUTPUT_DIR/"
cp -r assets "$OUTPUT_DIR/"

echo "Creating zip..."
(cd build && zip -r ../gravioli-windows.zip windows/)

echo "Done! Windows build is in $OUTPUT_DIR/ and gravioli-windows.zip"
