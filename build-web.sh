#!/bin/bash
set -euo pipefail

TARGET="wasm32-unknown-unknown"
OUTPUT_DIR="build/web"

# Ensure the Rust target is installed
if ! rustup target list --installed | grep -q "$TARGET"; then
    echo "Installing Rust target $TARGET..."
    rustup target add "$TARGET"
fi

echo "Building for $TARGET..."
cargo build --release --target "$TARGET"

echo "Packaging..."
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR"
cp "target/$TARGET/release/gravioli.wasm" "$OUTPUT_DIR/"
cp -r assets "$OUTPUT_DIR/"

# Download mq_js_bundle.js if not cached
if [ ! -f "$OUTPUT_DIR/mq_js_bundle.js" ]; then
    echo "Downloading mq_js_bundle.js..."
    curl -sL -o "$OUTPUT_DIR/mq_js_bundle.js" \
        https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js
fi

# Generate index.html
cat > "$OUTPUT_DIR/index.html" << 'HTMLEOF'
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Gravioli</title>
    <style>
        html,
        body,
        canvas {
            margin: 0;
            padding: 0;
            width: 100%;
            height: 100%;
            overflow: hidden;
            position: absolute;
            background: black;
            z-index: 0;
        }
    </style>
</head>
<body>
    <canvas id="glcanvas" tabindex='1'></canvas>
    <script src="mq_js_bundle.js"></script>
    <script>load("gravioli.wasm");</script>
</body>
</html>
HTMLEOF

echo "Done! Web build is in $OUTPUT_DIR/"
echo "To test locally: cargo install basic-http-server && basic-http-server $OUTPUT_DIR"
