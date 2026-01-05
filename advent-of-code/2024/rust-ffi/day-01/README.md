# Advent of Code 2024 - Day 1: FFI Learning Project

This project demonstrates various FFI (Foreign Function Interface) techniques in Rust, from basic C stdlib functions to external C libraries.

## Building

### With Nix (Recommended)

From the `rust-ffi` directory:
```bash
nix-shell
cd day-01
cargo build
cargo run
```

### Without Nix

You'll need:
- Rust toolchain
- `pkg-config` (for GLib)
- GLib development files

On macOS:
```bash
brew install pkg-config glib
```

On Linux (Debian/Ubuntu):
```bash
sudo apt install pkg-config libglib2.0-dev
```

Then:
```bash
cargo build
cargo run
```

## FFI Approaches Demonstrated

This project demonstrates **9 different FFI patterns** for interacting with C code from Rust:

### Part 1: Sorting Implementations

1. **Pure Rust Baseline** (`process_part1`)
   - Native Rust `.sort()` using timsort
   - Baseline for comparison

2. **Manual FFI - qsort** (`process_part1_c`)
   - Hand-written `extern "C"` block declaring `qsort`
   - Function pointers and C callbacks
   - Demonstrates: raw FFI, unsafe blocks, pointer casting

3. **libc crate - qsort** (`process_part1_libc`)
   - Using pre-made bindings from `libc` crate
   - Shows: community-maintained FFI bindings
   - Note: `libc` wraps function pointers in `Option<fn>`

### Part 2: Frequency Counting Implementations

4. **C-style Simulation** (`process_part2_c`)
   - Rust code mimicking C patterns
   - Manual pointer iteration with `.add()`
   - Demonstrates: C-style loops without actual FFI

5. **Binary Search with bsearch** (`process_part2_bsearch`)
   - Uses C's `bsearch` stdlib function
   - Manual pointer arithmetic to count range
   - Demonstrates: `.offset()`, `.offset_from()`, null pointer handling
   - Complexity: O(n log n + n*k) vs O(n²)

6. **Simulated Frequency Map** (`process_part2_freqmap`)
   - Rust implementation using `std::alloc`
   - Simulates C's `malloc`/`calloc`/`free`
   - Demonstrates: `Layout::array()`, manual memory management

7. **Real C Code - Custom Library** (`process_part2_glibc_count`, `process_part2_glibc_freqmap`)
   - Actual compiled C code from `aoc_ffi.c`
   - Build script compiles and links C files
   - Demonstrates: `cc` crate, `build.rs`, `#[repr(C)]` structs
   - Files: `aoc_ffi.h`, `aoc_ffi.c`

8. **GLib Hash Table** (`process_part2_glib`)
   - Using `glib-sys` crate for GLib bindings
   - GLib's `GHashTable` for frequency counting
   - Demonstrates: external system library FFI, `pkg-config` integration
   - Requires: GLib installed (via nix-shell or system package manager)

9. **uthash - Header-Only Library** (`process_part2_uthash`)
   - Vendored header-only C library in `vendor/uthash.h`
   - Custom C wrapper to expose uthash macros as functions
   - Demonstrates: vendoring C libraries, opaque pointers, zero-sized types
   - Files: `uthash_wrapper.h`, `uthash_wrapper.c`
   - No external dependencies needed!

---

## Key FFI Concepts Covered

### Memory Management
- Raw pointer manipulation (`.as_ptr()`, `.as_mut_ptr()`)
- Pointer arithmetic (`.add()`, `.offset()`, `.offset_from()`)
- Manual allocation (`std::alloc::alloc_zeroed()`, `malloc`/`calloc`)
- Manual deallocation (`std::alloc::dealloc()`, `free()`)
- Null pointer checking (`.is_null()`)

### Type Safety
- `#[repr(C)]` for C-compatible struct layout
- Opaque pointers with zero-sized types
- Function pointers and callbacks
- `unsafe extern "C"` blocks

### Build Integration
- `build.rs` with `cc` crate for compiling C code
- `.include()` for header directories
- `cargo:rerun-if-changed` for incremental builds
- Vendoring external C libraries

### External Libraries
- System libraries via `-sys` crates (`libc`, `glib-sys`)
- `pkg-config` for finding system libraries
- Header-only libraries (uthash)
- Custom C code compilation

---

### Why qsort needs a comparison function:

1. **C doesn't have generics or type information at runtime** - `qsort` receives a `void*` (just raw bytes). It has no idea what those bytes mean. Are they integers? Floats? Strings? Structs?
1. **Different types need different comparison logic**:
- `int`: arithmetic comparison (`a - b`)
- `float`: can't use subtraction (NaN, precision issues)
- `char*`: need `strcmp`, not pointer comparison
-Structs: which field(s) to compare?
1. **The bytes themselves aren't sortable** - Without knowing the type, you can't just compare raw bytes:
- For `i32`: the value `256` is `[0, 1, 0, 0]` in little-endian bytes
- For `i32`: the value `1` is `[1, 0, 0, 0]` in little-endian bytes
- Byte-wise comparison would say `1 > 256` (because `[1...] > [0...]`) which is wrong!
1. **Endianness matters** - Even if you tried byte comparison, multi-byte integers are stored differently on different architectures.

The comparison function bridges the gap:
- You (the programmer) know the type
- You cast `void*` back to the actual type
- You provide the semantic comparison logic for that type

This is why Rust's `.sort(`) is so much nicer - it uses generics and the `Ord` trait, so the compiler generates the right comparison code automatically. C's `qsort` makes you do this manually, which is more error-prone but also shows you exactly what's happening under the hood!


### Counting with C (pointer arithmetic)

- Passing array pointers across FFI boundary
- Manual length tracking (C doesn't have slices)
- Pointer arithmetic with .add()
- Safe wrapper pattern


### Binary Search with C

1. `bsearch` **FFI declaration** (lib.rs:49-60) - Imported C's binary search function
1. `c_bsearch_count` (lib.rs:181-220) - Complex FFI function that:
- Uses `bsearch` to find ANY occurrence of the target (O(log n))
- Returns null pointer if not found
- Uses `.offset(-1)` to walk backwards to find first occurrence
- Uses `.offset(1)` to walk forwards to find last occurrence
- Calculates count using `.offset_from()` pointer arithmetic
1. `process_part2_bsearch` (lib.rs:231-241) - Sorts with `qsort` first, then uses binary search

**Key FFI concepts demonstrated:**

- Using C stdlib's `bsearch` function
- Null pointer checking (`is_null()`)
- Pointer arithmetic with `.offset()` and `.offset_from()`
- Pointer comparison (`first > sorted_arr`)
- Combining multiple C functions (`qsort` + `bsearch`)

**Complexity improvement:**

- Original: O(n²) - for each left element, scan entire right array
- Linear count: O(n²) - same complexity, just using C
- Binary search: O(n log n + n*k) where k = avg occurrences - much better for large arrays!

---

## UniFFI: Multi-Language Bindings

This project uses **UniFFI** to generate bindings for Python, Kotlin, and Swift, allowing you to call all 9 FFI implementations from other languages!

### Quick Start

```bash
# Enter nix shell (sets up Python + uniffi-bindgen automatically)
nix-shell

# Generate and test Python bindings
just test-python

# Or generate all bindings at once
just gen-all
```

### Manual Setup (without Nix)

```bash
# Install uniffi-bindgen
pip install uniffi-bindgen==0.28.3

# Generate Python bindings
just gen-python

# Generate Kotlin bindings
just gen-kotlin

# Generate Swift bindings
just gen-swift
```

### Using from Python

```python
import sys
sys.path.insert(0, 'bindings/python')
import aoc_ffi_day01

# All 9 FFI implementations available!
result = aoc_ffi_day01.uniffi_process_part2_uthash("3   4\n4   3\n2   5")
print(f"Result: {result}")  # Result: 31
```

### Available Functions

All functions accept a `String` input and return `Result<i32, AocError>`:

**Part 1 (sorting):**
- `uniffi_process_part1()` - Pure Rust
- `uniffi_process_part1_c()` - Manual FFI qsort
- `uniffi_process_part1_libc()` - libc crate

**Part 2 (frequency counting):**
- `uniffi_process_part2()` - Pure Rust
- `uniffi_process_part2_c()` - C-style pointer iteration
- `uniffi_process_part2_bsearch()` - Binary search with bsearch
- `uniffi_process_part2_freqmap()` - Simulated C malloc/free
- `uniffi_process_part2_glibc_count()` - Real C code (linear count)
- `uniffi_process_part2_glibc_freqmap()` - Real C code (frequency map)
- `uniffi_process_part2_libc()` - libc crate bsearch
- `uniffi_process_part2_glib()` - GLib GHashTable
- `uniffi_process_part2_uthash()` - uthash header-only library

### Architecture

```
┌─────────────────────────────────────┐
│   Python / Kotlin / Swift Code      │
└──────────────┬──────────────────────┘
               │ UniFFI generated bindings
┌──────────────▼──────────────────────┐
│   Rust Wrapper Functions            │
│   (#[uniffi::export])                │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│   Core Rust FFI Implementations     │
│   (9 different approaches)           │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│   C Libraries                        │
│   (qsort, bsearch, GLib, uthash)    │
└─────────────────────────────────────┘
```

See [UNIFFI_SETUP.md](UNIFFI_SETUP.md) for detailed setup instructions.