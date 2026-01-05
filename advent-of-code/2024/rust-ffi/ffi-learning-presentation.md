---
title: FFI Learning Journey
sub_title: From Rust to C and Back Again
author: Advent of Code 2024, Day 1
---

# FFI Learning Journey

> Exploring Foreign Function Interfaces in Rust
>
> **Two Directions:**
> - Rust → C (9 implementations)
> - Rust → Other Languages (UniFFI)

<!-- end_slide -->

# The Problem

**Advent of Code 2024, Day 1:**

```
3   4
4   3
2   5
1   3
3   9
3   3
```

**Part 1:** Sort both columns, sum absolute differences
**Part 2:** Calculate similarity score (frequency counting)

*Simple enough to understand, complex enough to learn FFI*

<!-- end_slide -->

# Git History: The Journey

```
step 0              # Project setup
step 1 - rust       # Pure Rust baseline
step 2 - c qsort    # First FFI: manual extern "C"
step 3 - benchmark  # Measure performance
step 4 - generic    # Abstraction patterns
step 5 - part 2     # Second problem
```

Then the FFI exploration began...

<!-- end_slide -->

# Rust → C: 9 Approaches

## Part 1: Sorting (3 approaches)

1. **Pure Rust** - Baseline using `.sort()`
2. **Manual FFI** - Hand-written `extern "C"` for qsort
3. **libc crate** - Community-maintained bindings

<!-- end_slide -->

# Manual FFI: qsort

```rust
// Declare C function
extern "C" {
    fn qsort(
        base: *mut c_void,
        nmemb: size_t,
        size: size_t,
        compar: extern "C" fn(*const c_void, *const c_void) -> c_int,
    );
}

// Comparison function
extern "C" fn compare_i32(a: *const c_void, b: *const c_void) -> c_int {
    unsafe {
        let a = *(a as *const i32);
        let b = *(b as *const i32);
        a - b
    }
}
```

**Key concepts:** Function pointers, unsafe blocks, pointer casting

<!-- end_slide -->

# Why Does qsort Need a Comparison Function?

**C has no generics or runtime type info**

```c
void qsort(void *base, size_t nmemb, size_t size,
           int (*compar)(const void *, const void *));
```

- `void*` is just raw bytes - could be int, float, string, struct...
- Bytes aren't sortable: `256` = `[0,1,0,0]`, `1` = `[1,0,0,0]`
- Byte comparison would say `1 > 256` (wrong!)

**Your comparison function:**
- Casts `void*` back to actual type
- Provides semantic comparison logic
- Bridges the gap between raw bytes and meaning

<!-- end_slide -->

# Part 2: Frequency Counting (6 approaches)

4. **C-style simulation** - Rust mimicking C patterns
5. **bsearch** - Binary search for O(n log n)
6. **Simulated malloc/free** - Manual memory with `std::alloc`
7. **Real C code** - Compiled C library via `build.rs`
8. **GLib hash table** - External system library
9. **uthash** - Vendored header-only library

<!-- end_slide -->

# Binary Search: bsearch

```rust
extern "C" {
    fn bsearch(
        key: *const c_void,
        base: *const c_void,
        nmemb: size_t,
        size: size_t,
        compar: extern "C" fn(*const c_void, *const c_void) -> c_int,
    ) -> *mut c_void;
}
```

**Complexity improvement:**
- Original: O(n²) - scan entire array for each element
- Binary search: O(n log n + n×k) - much better!

**FFI concepts:** Null pointer handling, `.offset()`, `.offset_from()`

<!-- end_slide -->

# Real C Code: build.rs

```rust
// build.rs
fn main() {
    cc::Build::new()
        .file("src/aoc_ffi.c")
        .include("src")
        .compile("aoc_ffi");

    println!("cargo:rerun-if-changed=src/aoc_ffi.c");
}
```

**Demonstrates:**
- `cc` crate for compiling C code
- `#[repr(C)]` structs for ABI compatibility
- Seamless integration with Cargo build

<!-- end_slide -->

# GLib: External System Library

```toml
[dependencies]
glib-sys = { version = "0.20", optional = true }

[features]
default = ["glib"]
glib = ["glib-sys"]
```

```rust
#[cfg(feature = "glib")]
use glib_sys::{
    g_hash_table_new, g_hash_table_destroy,
    g_hash_table_insert, g_hash_table_lookup,
    g_direct_hash, g_direct_equal
};
```

**Why optional?** Swift compatibility! (More on this later...)

<!-- end_slide -->

# uthash: Header-Only Library

**Vendored in `vendor/uthash.h`** (2,600 lines of C macros!)

```c
// uthash_wrapper.c - Expose macros as functions
struct FreqEntry* uthash_find(struct FreqEntry *head, int num);
void uthash_add(struct FreqEntry **head, int num);
void uthash_free_all(struct FreqEntry *head);
```

**Advantages:**
- No external dependencies
- Zero runtime overhead
- Industry-proven (used in major projects)

<!-- end_slide -->

# FFI Concepts Covered

**Memory Management:**
- Raw pointers (`.as_ptr()`, `.as_mut_ptr()`)
- Pointer arithmetic (`.add()`, `.offset()`)
- Manual allocation/deallocation
- Null pointer checking

**Type Safety:**
- `#[repr(C)]` for C-compatible layout
- Opaque pointers with zero-sized types
- Function pointers and callbacks
- `unsafe extern "C"` blocks

<!-- end_slide -->

# Now... The Other Direction

# Rust → Other Languages

**Challenge:** We've mastered Rust calling C
**Goal:** Let other languages call our Rust!

**Enter UniFFI** 🎉

<!-- end_slide -->

# What is UniFFI?

**Mozilla's Uniform FFI Generator**

```
         ┌─────────────┐
         │   Rust      │
         │   Code      │
         └──────┬──────┘
                │
         ┌──────▼──────┐
         │   UniFFI    │
         │   .udl      │
         └──────┬──────┘
                │
     ┌──────────┼──────────┐
     ▼          ▼          ▼
  Python     Kotlin     Swift
```

**One interface definition → Multiple language bindings**

<!-- end_slide -->

# UniFFI Setup: The Commits

```
uniffi                  # Initial exploration
WIP: uniffi - python    # First language: Python
```

**Challenge #1:** Version compatibility
- UniFFI 0.30 not on PyPI
- Rust 2024 edition incompatible with UniFFI 0.28

**Solution:** Downgrade to Rust 2021 + UniFFI 0.28

```
Downgrade to Rust 2021 edition and UniFFI 0.28 for compatibility
```

<!-- end_slide -->

# Python Bindings: Success! ✅

```python
import sys
sys.path.insert(0, 'bindings/python')
import aoc_ffi_day01

# All 9 FFI implementations available!
result = aoc_ffi_day01.uniffi_process_part2_uthash(
    "3   4\n4   3\n2   5"
)
print(f"Result: {result}")  # Result: 31
```

**Testing:**
```bash
nix-shell
just test-python
```

All 12 functions working! 🎉

<!-- end_slide -->

# Kotlin: The Journey

```
WIP: uniffi - kotlin & swift  (3 commits)
uniffi - kotlin & swift       (2 commits)
```

**Challenge #2:** Functions not found in Kotlin

**Root cause:** Missing JNA (Java Native Access)

**Solution:**
- Add JNA to nix devshell
- Auto-download from Maven
- Set `CLASSPATH` automatically

<!-- end_slide -->

# Kotlin Bindings: Success! ✅

```kotlin
import uniffi.aoc_ffi_day01.*

val result = uniffiProcessPart2Uthash(
    "3   4\n4   3\n2   5"
)
println("Result: $result")  // Result: 31
```

**Testing:**
```bash
nix-shell
just test-kotlin
```

All 12 functions working! 🎉

<!-- end_slide -->

# Swift: The Challenge

```
WIP: uniffi - swift  (5 commits)
FAIL: uniffi - swift (1 commit)
```

**Challenge #3:** Swift SDK incompatibility

**The Problem:**
- Nix SDK: v11.3 (Swift 5.4)
- System Swift: v6.2.1
- GLib from nix store: Not available outside nix-shell

<!-- end_slide -->

# Swift: The Attempted Solution

**Idea:** Make GLib optional

```toml
[features]
default = ["glib"]
glib = ["glib-sys"]
```

```rust
#[cfg(feature = "glib")]
pub fn process_part2_glib(...) -> Result<i32, AocError> {
    // GLib implementation
}
```

**Build without GLib:**
```bash
cargo build --release --lib --no-default-features
```

<!-- end_slide -->

# Swift: The Reality

**Workflow:**
1. Generate bindings in nix-shell: `just gen-swift` ✅
2. Test outside nix-shell: `just test-swift` ❌

**Result:** Segmentation fault (exit code 139)

**Even with:**
- No GLib dependency
- System-only library paths
- Clean dylib build

**Conclusion:** Deeper incompatibility between Rust (from nix) and Swift runtime

<!-- end_slide -->

# Swift: The Documentation

Created comprehensive docs:

- **SWIFT_WORKFLOW.md** - Step-by-step process
- **SWIFT_LIMITATIONS.md** - Technical analysis
- **README.md** - Clear status summary

**Status:** ⚠️ Partial Support
- Bindings generate successfully
- 11 of 12 implementations available (GLib excluded)
- Tests fail at runtime

<!-- end_slide -->

# UniFFI Architecture

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

<!-- end_slide -->

# The Complete Picture

**Rust → C (9 implementations):**
1. Pure Rust baseline
2. Manual FFI (qsort)
3. libc crate
4. C-style simulation
5. bsearch binary search
6. Simulated malloc/free
7. Real C code (custom library)
8. GLib (system library)
9. uthash (header-only)

**Rust → Other Languages (UniFFI):**
- ✅ Python (all 12 functions)
- ✅ Kotlin (all 12 functions)
- ⚠️ Swift (generates but doesn't run)

<!-- end_slide -->

# Key Takeaways

**FFI is Complex:**
- Memory management across boundaries
- ABI compatibility (`#[repr(C)]`)
- Type safety with unsafe code
- Platform-specific challenges

**Build Systems Matter:**
- `build.rs` is powerful
- Nix provides reproducibility
- But can create isolation challenges

**Documentation is Critical:**
- What works
- What doesn't work
- **Why** it doesn't work

<!-- end_slide -->

# Lessons Learned

**Start Simple:**
- Pure Rust baseline first
- Add complexity gradually
- Benchmark to understand tradeoffs

**Understand the Tools:**
- C has no generics → need comparison functions
- Rust has safety → need `unsafe` blocks
- UniFFI bridges languages → but has limitations

**Environment Matters:**
- Nix vs system dependencies
- SDK versions
- Platform-specific toolchains

<!-- end_slide -->

# Resources

**Project Structure:**
```
rust-ffi/
├── day-01/
│   ├── src/
│   │   ├── lib.rs          # 9 FFI implementations
│   │   ├── aoc_ffi.c       # Custom C code
│   │   ├── uthash_wrapper.c # uthash bindings
│   │   └── aoc_ffi_day01.udl # UniFFI interface
│   ├── vendor/
│   │   └── uthash.h        # Vendored library
│   └── bindings/
│       ├── python/
│       ├── kotlin/
│       └── swift/
├── justfile                 # Task runner
└── default.nix             # Nix environment
```

<!-- end_slide -->

# Quick Start

```bash
# Enter development environment
nix-shell

# Run all FFI implementations
just run

# Test Python bindings
just test-python

# Test Kotlin bindings
just test-kotlin

# Generate Swift bindings (tests don't work yet)
just gen-swift
```

<!-- end_slide -->

# Future Work

**Swift Bindings:**
- Investigate root cause of segfault
- Try building outside nix entirely
- Consider alternative approaches (framework bundling, static linking)

**More Languages:**
- Ruby (UniFFI support exists)
- Go (via cgo)
- JavaScript (via WASM or N-API)

**More FFI Patterns:**
- Callbacks from C to Rust
- Shared memory
- Multi-threading across FFI boundary

<!-- end_slide -->

# Questions?

**Project demonstrates:**
- ✅ 9 different Rust → C FFI patterns
- ✅ Python bindings via UniFFI
- ✅ Kotlin bindings via UniFFI
- ⚠️ Swift bindings (partial)

**Key insight:** FFI is powerful but requires careful attention to:
- Memory safety
- ABI compatibility
- Platform differences
- Toolchain integration

<!-- end_slide -->

# Thank You!

> *"The best way to learn FFI is to write it nine different ways"*
>
> — This project, probably

**All code and docs available in the repository**

*Special thanks to Nix, Rust, UniFFI, and the spirit of learning through experimentation (mostly breaking things)*
