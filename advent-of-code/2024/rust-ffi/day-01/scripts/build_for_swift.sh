#!/bin/bash
set -e

# Build Rust library for Swift bindings (without nix dependencies)
# This script must be run OUTSIDE of nix-shell

# Check if we're in nix-shell
if [ -n "$IN_NIX_SHELL" ]; then
    echo "⚠️  Error: This script must be run OUTSIDE nix-shell"
    echo "   Nix dependencies (like GLib) are incompatible with system Swift"
    echo "   Please exit nix-shell first, then run this script again"
    exit 1
fi

echo "Building Rust library for Swift (without nix dependencies)..."
echo ""

# Temporarily disable glib feature by commenting out glib-sys in Cargo.toml
# Note: This means GLib-based FFI functions won't work, but the library will compile
echo "Note: GLib-based implementations will be unavailable in Swift tests"
echo "      (process_part2_glib will not work due to system incompatibility)"
echo ""

# Build the library
cargo build --release --lib

echo ""
echo "✓ Rust library built successfully"
echo "  Library: target/release/libaoc_ffi_day01.dylib"
echo ""
echo "Now you can:"
echo "  1. Generate Swift bindings: just gen-swift (must be in nix-shell for uniffi-bindgen)"
echo "  2. Test Swift bindings: ./test_swift_bindings.sh (must be OUTSIDE nix-shell)"
