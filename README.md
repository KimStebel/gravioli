# Gravioli

A gravity-based puzzle game where you pilot a rocket through space, using gravitational forces from planets and limited engine thrust to reach goal zones.

## Building

Requires Rust (2024 edition). On Linux, `libasound2-dev` and `pkg-config` are also needed for audio support.

```sh
cargo build
```

### Cross-compilation

**Web (WASM):**

```sh
cargo build --target wasm32-unknown-unknown --release
```

**Windows (MinGW):**

```sh
cargo build --target x86_64-pc-windows-gnu --release
```

## Testing

```sh
cargo test
```

## Running

```sh
cargo run
```

## Deployment

### Web

Build the WASM binary and assemble the web bundle:

```sh
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/gravioli.wasm build/web/
```

The `build/web/` directory contains everything needed to serve the game: `index.html`, `mq_js_bundle.js`, the `.wasm` binary, and the `assets/` folder. Upload the entire directory to any static file host (e.g. GitHub Pages, Netlify, S3, or a basic Nginx/Apache server).

### Windows

Build the Windows binary and assemble the distribution folder:

```sh
cargo build --target x86_64-pc-windows-gnu --release
cp target/x86_64-pc-windows-gnu/release/gravioli.exe build/windows/
```

Distribute the `build/windows/` directory (the executable plus the `assets/` folder) as a zip archive.

### Linux

Build a release binary:

```sh
cargo build --release
```

Run the resulting binary from the project root (so the `assets/` directory is found):

```sh
./target/release/gravioli
```

