# Building NUX Launcher on Debian Linux

This document details how to compile NUX Launcher from source on Debian 13 (Trixie) or Debian 12 (Bookworm).

---

## Prerequisites

Ensure you have Rust stable (1.80+ recommended) and the required native development libraries:

```bash
# Debian packages
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libgtk-4-dev \
    libglib2.0-dev \
    libcairo2-dev \
    libpango1.0-dev \
    libgraphene-1.0-dev \
    openjdk-17-jre
```

*Note: SQLite is compiled natively using the bundled `rusqlite` feature, so `libsqlite3-dev` is not required.*

---

## Compilation

Clone the repository and build using Cargo:

### Debug Build (Fast compilation):
```bash
cargo build
```

### Optimized Release Build:
```bash
cargo build --release
```

The release profile enables:
- `opt-level = 3` (Maximum compiler optimizations)
- `lto = "thin"` (Link-Time Optimization across translation units)
- `codegen-units = 1` (Whole-crate optimization)
- `panic = "abort"` (Eliminates stack unwinding overhead)
- `strip = true` (Strips debug symbols to reduce binary size to ~18 MB)

The compiled binary will be placed at:
```bash
target/release/nux-launcher
```

---

## Running the Unit Tests

Execute the comprehensive test suite:

```bash
cargo test
```

This verifies:
1. `java_test`: Probes system JVM and `.minecraft/runtime` for Java 17+ 64-bit compatibility.
2. `manifest_test`: Tests Mojang manifest parsing, rule evaluation, and maven coordinates.
3. `crypto_test`: Tests SHA-1 verification against sample files and byte streams.
4. `presets_test`: Validates Balanced and Low Resource memory and render distance limits.
5. `mods_test`: Tests automatic identification of Sodium, Iris, Lithium, and FerriteCore.
