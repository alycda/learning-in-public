#!/bin/bash
set -e

# Test Swift bindings for Advent of Code 2024 Day 1
# This script must be run OUTSIDE of nix-shell due to Swift SDK incompatibility

# Check if we're in nix-shell
if [ -n "$IN_NIX_SHELL" ]; then
    echo "⚠️  Error: This script must be run OUTSIDE nix-shell"
    echo "   Please exit nix-shell first, then run this script again"
    exit 1
fi

# Check if Swift bindings have been generated
if [ ! -f "bindings/swift/aoc_ffi_day01.swift" ]; then
    echo "⚠️  Error: Swift bindings not found"
    echo "   Please generate them first by running in nix-shell:"
    echo "   just gen-swift"
    exit 1
fi

echo "Testing Swift bindings..."
echo "Note: This will compile and run the Swift test binary"
echo ""

cd bindings/swift

# Compile the Swift test binary
echo "Compiling Swift test binary..."
swiftc -o test_swift \
    ../../test_swift_bindings.swift \
    aoc_ffi_day01.swift \
    -import-objc-header aoc_ffi_day01FFI.h \
    -L . \
    -laoc_ffi_day01

# Set DYLD_LIBRARY_PATH to find the dylib in current directory
echo "Running Swift tests..."
DYLD_LIBRARY_PATH=.:$DYLD_LIBRARY_PATH ./test_swift

# Clean up the binary
rm -f test_swift

echo ""
echo "✓ Swift tests completed"
