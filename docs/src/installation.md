# Installation

## Prerequisites

- Rust toolchain (edition 2024)

## Install from crates.io

```bash
cargo install rsimagetag
```

This downloads, compiles, and installs the latest published version into `~/.cargo/bin/`.

## Building from Source

```bash
git clone https://github.com/veltzer/rsimagetag.git
cd rsimagetag
cargo build --release
```

The binary will be at `target/release/rsimagetag`.

## Install Directly

```bash
cargo install --path .
```
