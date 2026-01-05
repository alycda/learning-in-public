#!/bin/bash
# Generate UniFFI bindings for Python, Kotlin, and Swift

set -e

# Build the library first
cargo build --release --lib

# Create output directories
mkdir -p bindings/python
mkdir -p bindings/kotlin
mkdir -p bindings/swift

# Generate Python bindings
cargo run -p uniffi --bin uniffi-bindgen generate \
  --library target/release/libaoc_ffi_day01.dylib \
  --language python \
  --out-dir bindings/python

echo "Python bindings generated in bindings/python/"
