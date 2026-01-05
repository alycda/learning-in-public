---
title: FFI Learning Journey
sub_title: From Rust to C and Back Again
author: Advent of Code 2024, Day 1
---

<!-- jump_to_middle -->

# FFI Learning Journey

> Exploring Foreign Function Interfaces in Rust

<!-- pause -->

**Two Directions:**
<!-- incremental_lists: true -->
- Rust → C (9 implementations)
- Rust → Other Languages (UniFFI)
<!-- incremental_lists: false -->

<!-- end_slide -->

# The Problem

<!-- column_layout: [1, 2, 1] -->
<!-- column: 1 -->

**Advent of Code 2024, Day 1:**

```
3   4
4   3
2   5
1   3
3   9
3   3
```

<!-- pause -->

**Part 1:** Sort both columns, sum absolute differences

**Part 2:** Calculate similarity score (frequency counting)

<!-- pause -->

*Simple enough to understand, complex enough to learn FFI*

<!-- end_slide -->

# Git History: The Journey

```bash
step 0              # Project setup
step 1 - rust       # Pure Rust baseline
step 2 - c qsort    # First FFI: manual extern "C"
step 3 - benchmark  # Measure performance
step 4 - generic    # Abstraction patterns
step 5 - part 2     # Second problem
```

<!-- pause -->

Then the FFI exploration began...

<!-- end_slide -->

# Rust → C: 9 Approaches

<!-- jump_to_middle -->

## Part 1: Sorting (3 approaches)

<!-- incremental_lists: true -->
1. **Pure Rust** - Baseline using `.sort()`
2. **Manual FFI** - Hand-written `extern "C"` for qsort
3. **libc crate** - Community-maintained bindings
<!-- incremental_lists: false -->

<!-- end_slide -->

# Manual FFI: qsort

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

**Declare C function:**

```rust
extern "C" {
    fn qsort(
        base: *mut c_void,
        nmemb: size_t,
        size: size_t,
        compar: extern "C" fn(
            *const c_void,
            *const c_void
        ) -> c_int,
    );
}
```

<!-- column: 1 -->

**Comparison function:**

```rust
extern "C" fn compare_i32(
    a: *const c_void,
    b: *const c_void
) -> c_int {
    unsafe {
        let a = *(a as *const i32);
        let b = *(b as *const i32);
        a - b
    }
}
```

<!-- reset_layout -->

<!-- pause -->

**Key concepts:** Function pointers, unsafe blocks, pointer casting

<!-- end_slide -->

# Why Does qsort Need a Comparison Function?

<!-- jump_to_middle -->

**C has no generics or runtime type info**

```c
void qsort(void *base, size_t nmemb, size_t size,
           int (*compar)(const void *, const void *));
```

<!-- pause -->

<!-- incremental_lists: true -->
- `void*` is just raw bytes - could be int, float, string, struct...
- Bytes aren't sortable: `256` = `[0,1,0,0]`, `1` = `[1,0,0,0]`
- Byte comparison would say `1 > 256` (wrong!)
<!-- incremental_lists: false -->

<!-- pause -->

**Your comparison function:**
- Casts `void*` back to actual type
- Provides semantic comparison logic
- Bridges the gap between raw bytes and meaning

<!-- end_slide -->

# Part 2: Frequency Counting

## 6 Different Approaches

<!-- incremental_lists: true -->
4. **C-style simulation** - Rust mimicking C patterns
5. **bsearch** - Binary search for O(n log n)
6. **Simulated malloc/free** - Manual memory with `std::alloc`
7. **Real C code** - Compiled C library via `build.rs`
8. **GLib hash table** - External system library
9. **uthash** - Vendored header-only library
<!-- incremental_lists: false -->

<!-- end_slide -->

# Binary Search: bsearch

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

```rust
extern "C" {
    fn bsearch(
        key: *const c_void,
        base: *const c_void,
        nmemb: size_t,
        size: size_t,
        compar: extern "C" fn(
            *const c_void,
            *const c_void
        ) -> c_int,
    ) -> *mut c_void;
}
```

<!-- column: 1 -->

**Complexity improvement:**
- <span style="color:red">Original: O(n²)</span>
- <span style="color:green">Binary search: O(n log n + n×k)</span>

<!-- pause -->

**FFI concepts:**
- Null pointer handling
- `.offset()`
- `.offset_from()`

<!-- end_slide -->

# Real C Code: build.rs

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

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

<!-- column: 1 -->

**Demonstrates:**

<!-- incremental_lists: true -->
- `cc` crate for compiling C
- `#[repr(C)]` structs
- Seamless Cargo integration
<!-- incremental_lists: false -->

<!-- end_slide -->

# GLib: External System Library

```toml
[dependencies]
glib-sys = { version = "0.20", optional = true }

[features]
default = ["glib"]
glib = ["glib-sys"]
```

<!-- pause -->

```rust
#[cfg(feature = "glib")]
use glib_sys::{
    g_hash_table_new, g_hash_table_destroy,
    g_hash_table_insert, g_hash_table_lookup,
    g_direct_hash, g_direct_equal
};
```

<!-- pause -->

**Why optional?** <span>Swift compatibility!</span> (More on this later...)

<!-- end_slide -->

# uthash: Header-Only Library

**Vendored in `vendor/uthash.h`** (2,600 lines of C macros!)

<!-- column_layout: [3, 2] -->

<!-- column: 0 -->

```c
// uthash_wrapper.c
// Expose macros as functions
struct FreqEntry* uthash_find(
    struct FreqEntry *head,
    int num
);

void uthash_add(
    struct FreqEntry **head,
    int num
);

void uthash_free_all(
    struct FreqEntry *head
);
```

<!-- column: 1 -->

<!-- pause -->

**Advantages:**

<!-- incremental_lists: true -->
- No external dependencies
- Zero runtime overhead
- Industry-proven
<!-- incremental_lists: false -->

<!-- end_slide -->

# FFI Concepts Covered

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

**Memory Management:**
<!-- incremental_lists: true -->
- Raw pointers
- Pointer arithmetic
- Manual allocation
- Null checking
<!-- incremental_lists: false -->

<!-- column: 1 -->

**Type Safety:**
<!-- incremental_lists: true -->
- `#[repr(C)]` layout
- Opaque pointers
- Function pointers
- `unsafe extern "C"`
<!-- incremental_lists: false -->

<!-- end_slide -->

<!-- jump_to_middle -->

# Now... The Other Direction

# Rust → Other Languages

<!-- pause -->

**Challenge:** We've mastered Rust calling C

**Goal:** Let other languages call our Rust!

<!-- pause -->

**Enter UniFFI** 🎉

<!-- end_slide -->

# What is UniFFI?

<!-- jump_to_middle -->

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

<!-- pause -->

**One interface definition → Multiple language bindings**

<!-- end_slide -->

# UniFFI Setup: The Commits

```bash
uniffi                  # Initial exploration
WIP: uniffi - python    # First language: Python
```

<!-- pause -->

**Challenge #1:** Version compatibility

<!-- incremental_lists: true -->
- UniFFI 0.30 not on PyPI
- Rust 2024 edition incompatible with UniFFI 0.28
<!-- incremental_lists: false -->

<!-- pause -->

**Solution:** Downgrade to Rust 2021 + UniFFI 0.28

```bash
Downgrade to Rust 2021 edition and UniFFI 0.28 for compatibility
```

<!-- end_slide -->

# Python Bindings: Success! ✅

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

```python
import sys
sys.path.insert(0, 'bindings/python')
import aoc_ffi_day01

# All 9 FFI implementations!
result = aoc_ffi_day01\
    .uniffi_process_part2_uthash(
    "3   4\n4   3\n2   5"
)
print(f"Result: {result}")
# Result: 31
```

<!-- column: 1 -->

**Testing:**
```bash
nix-shell
just test-python
```

<!-- pause -->

<span style="color:green">All 12 functions working! 🎉</span>

<!-- end_slide -->

# Kotlin: The Journey

```bash
WIP: uniffi - kotlin & swift  (3 commits)
uniffi - kotlin & swift       (2 commits)
```

<!-- pause -->

**Challenge #2:** Functions not found in Kotlin

<!-- pause -->

**Root cause:** Missing JNA (Java Native Access)

<!-- pause -->

**Solution:**
<!-- incremental_lists: true -->
- Add JNA to nix devshell
- Auto-download from Maven
- Set `CLASSPATH` automatically
<!-- incremental_lists: false -->

<!-- end_slide -->

# Kotlin Bindings: Success! ✅

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

```kotlin
import uniffi.aoc_ffi_day01.*

val result =
    uniffiProcessPart2Uthash(
    "3   4\n4   3\n2   5"
)
println("Result: $result")
// Result: 31
```

<!-- column: 1 -->

**Testing:**
```bash
nix-shell
just test-kotlin
```

<!-- pause -->

<span style="color:green">All 12 functions working! 🎉</span>

<!-- end_slide -->

# Swift: The Challenge

```bash
WIP: uniffi - swift  (5 commits)
FAIL: uniffi - swift (1 commit)
```

<!-- pause -->

**Challenge #3:** Swift SDK incompatibility

<!-- pause -->

**The Problem:**
<!-- incremental_lists: true -->
- Nix SDK: v11.3 (Swift 5.4)
- System Swift: v6.2.1
- GLib from nix store: Not available outside nix-shell
<!-- incremental_lists: false -->

<!-- end_slide -->

# Swift: The Attempted Solution

**Idea:** Make GLib optional

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

```toml
[features]
default = ["glib"]
glib = ["glib-sys"]
```

```rust
#[cfg(feature = "glib")]
pub fn process_part2_glib(
    ...
) -> Result<i32, AocError> {
    // GLib implementation
}
```

<!-- column: 1 -->

<!-- pause -->

**Build without GLib:**
```bash
cargo build --release \
    --lib \
    --no-default-features
```

<!-- pause -->

This removes nix dependencies!

<!-- end_slide -->

# Swift: The Reality

<!-- jump_to_middle -->

**Workflow:**
1. Generate bindings in nix-shell: `just gen-swift` ✅
2. Test outside nix-shell: `just test-swift` <span style="color:red">❌</span>

<!-- pause -->

**Result:** <span style="color:red">Segmentation fault (exit code 139)</span>

<!-- pause -->

**Even with:**
- No GLib dependency
- System-only library paths
- Clean dylib build

<!-- pause -->

**Conclusion:** Deeper incompatibility between Rust (from nix) and Swift runtime

<!-- end_slide -->

# Swift: The Documentation

<!-- jump_to_middle -->

Created comprehensive docs:

<!-- incremental_lists: true -->
- **SWIFT_WORKFLOW.md** - Step-by-step process
- **SWIFT_LIMITATIONS.md** - Technical analysis
- **README.md** - Clear status summary
<!-- incremental_lists: false -->

<!-- pause -->

**Status:** <span>⚠️ Partial Support</span>
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

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

**Rust → C (9 implementations):**

<!-- incremental_lists: true -->
1. Pure Rust baseline
2. Manual FFI (qsort)
3. libc crate
4. C-style simulation
5. bsearch binary search
6. Simulated malloc/free
7. Real C code
8. GLib (system library)
9. uthash (header-only)
<!-- incremental_lists: false -->

<!-- column: 1 -->

**Rust → Other Languages:**

<!-- pause -->

- <span style="color:green">✅ Python</span> (all 12 functions)
- <span style="color:green">✅ Kotlin</span> (all 12 functions)
- <span>⚠️ Swift</span> (generates but doesn't run)

<!-- end_slide -->

# Key Takeaways

<!-- jump_to_middle -->

**FFI is Complex:**
<!-- incremental_lists: true -->
- Memory management across boundaries
- ABI compatibility (`#[repr(C)]`)
- Type safety with unsafe code
- Platform-specific challenges
<!-- incremental_lists: false -->

<!-- pause -->

**Build Systems Matter:**
- Nix provides reproducibility
- But can create isolation challenges

<!-- pause -->

**Documentation is Critical:**
- What works, what doesn't, and **why**

<!-- end_slide -->

# Lessons Learned

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

**Start Simple:**
- Pure Rust baseline first
- Add complexity gradually
- Benchmark to understand

<!-- pause -->

**Understand the Tools:**
- C has no generics
- Rust requires `unsafe`
- UniFFI has limitations

<!-- column: 1 -->

**Environment Matters:**
- Nix vs system dependencies
- SDK versions
- Platform toolchains

<!-- pause -->

**Document Everything:**
- Successes
- Failures
- Workarounds

<!-- end_slide -->

# Resources

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

**Project Structure:**
```
rust-ffi/
├── day-01/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── aoc_ffi.c
│   │   └── uthash_wrapper.c
│   ├── vendor/
│   │   └── uthash.h
│   └── bindings/
│       ├── python/
│       ├── kotlin/
│       └── swift/
```

<!-- column: 1 -->

**Commands:**
```bash
# All implementations
just run

# Python
just test-python

# Kotlin
just test-kotlin

# Swift (generates only)
just gen-swift
```

<!-- end_slide -->

# Quick Start

<!-- jump_to_middle -->

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

<!-- column_layout: [1, 1] -->

<!-- column: 0 -->

**Swift Bindings:**
- Investigate segfault
- Try non-nix build
- Framework bundling
- Static linking

<!-- pause -->

**More Languages:**
- Ruby (UniFFI)
- Go (cgo)
- JavaScript (WASM)

<!-- column: 1 -->

<!-- pause -->

**More FFI Patterns:**
- C → Rust callbacks
- Shared memory
- Multi-threading

<!-- pause -->

**Performance:**
- Detailed benchmarks
- Optimization
- Trade-off analysis

<!-- end_slide -->

# Questions?

<!-- jump_to_middle -->

**Project demonstrates:**
<!-- incremental_lists: true -->
- <span style="color:green">✅ 9 different Rust → C FFI patterns</span>
- <span style="color:green">✅ Python bindings via UniFFI</span>
- <span style="color:green">✅ Kotlin bindings via UniFFI</span>
- <span>⚠️ Swift bindings (partial)</span>
<!-- incremental_lists: false -->

<!-- pause -->

**Key insight:** FFI is powerful but requires careful attention to:
Memory safety • ABI compatibility • Platform differences • Toolchain integration

<!-- end_slide -->

<!-- jump_to_middle -->

# Thank You!

> *"The best way to learn FFI is to write it nine different ways"*
>
> — This project, probably

<!-- pause -->

**All code and docs available in the repository**

<!-- pause -->

*Special thanks to Nix, Rust, UniFFI, and the spirit of learning through experimentation*

*(mostly breaking things)*

<!-- end_slide -->
