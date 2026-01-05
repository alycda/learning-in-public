# UniFFI Python Bindings Setup

This document explains how to generate and use Python bindings for the Rust FFI library.

## Prerequisites

1. **Rust library built**: The Rust library must be compiled first
   ```bash
   cargo build --release --lib
   ```

2. **Install uniffi-bindgen**: Python tool to generate bindings
   ```bash
   pip install uniffi-bindgen==0.28.3
   ```

   Note: We use uniffi-bindgen 0.28.3 (matches our Rust uniffi crate version)

## Generating Python Bindings

### Method 1: Using the Python script

```bash
python3 bindings_gen.py
```

### Method 2: Manual generation

```bash
# Make sure library is built
cargo build --release --lib

# Generate bindings
uniffi-bindgen generate src/aoc_ffi_day01.udl \
  --lib-file target/release/libaoc_ffi_day01.dylib \
  --language python \
  --out-dir bindings/python
```

## Using the Python Bindings

Once generated, you can use the library from Python:

```python
# Add bindings to path
import sys
sys.path.insert(0, 'bindings/python')

# Import the module
import aoc_ffi_day01

# Use the functions
sample_input = """3   4
4   3
2   5
1   3
3   9
3   3"""

# Test all implementations
print("Part 1 (Rust):", aoc_ffi_day01.uniffi_process_part1(sample_input))
print("Part 1 (C):", aoc_ffi_day01.uniffi_process_part1_c(sample_input))
print("Part 1 (libc):", aoc_ffi_day01.uniffi_process_part1_libc(sample_input))

print("\nPart 2 implementations:")
print("  Rust:", aoc_ffi_day01.uniffi_process_part2(sample_input))
print("  C-style:", aoc_ffi_day01.uniffi_process_part2_c(sample_input))
print("  Binary search:", aoc_ffi_day01.uniffi_process_part2_bsearch(sample_input))
print("  FreqMap:", aoc_ffi_day01.uniffi_process_part2_freqmap(sample_input))
print("  GLib C:", aoc_ffi_day01.uniffi_process_part2_glibc_count(sample_input))
print("  GLib FreqMap:", aoc_ffi_day01.uniffi_process_part2_glibc_freqmap(sample_input))
print("  libc:", aoc_ffi_day01.uniffi_process_part2_libc(sample_input))
print("  GLib hash:", aoc_ffi_day01.uniffi_process_part2_glib(sample_input))
print("  uthash:", aoc_ffi_day01.uniffi_process_part2_uthash(sample_input))
```

## Error Handling

Functions can raise `AocError` exceptions:

```python
try:
    result = aoc_ffi_day01.uniffi_process_part1("invalid input")
except aoc_ffi_day01.AocError as e:
    print(f"Error: {e}")
```

## Next Steps

- **Kotlin bindings**: Use `uniffi-bindgen-kotlin` to generate Kotlin bindings
- **Swift bindings**: Use `uniffi-bindgen-swift` to generate Swift bindings

## Troubleshooting

### uniffi-bindgen not found
```bash
pip install uniffi-bindgen==0.28.3
```

### Library not found
Make sure you built the release version:
```bash
cargo build --release --lib
```

### Wrong architecture
On macOS, the library is built for the native architecture. If you need cross-compilation, use:
```bash
cargo build --release --target aarch64-apple-darwin  # for ARM64
cargo build --release --target x86_64-apple-darwin   # for Intel
```
