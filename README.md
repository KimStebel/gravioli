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

