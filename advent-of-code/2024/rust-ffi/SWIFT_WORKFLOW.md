# Swift Bindings Workflow

The Swift bindings require a special workflow due to SDK incompatibility between nix and system Swift.

## Quick Start

```bash
# Step 1: Generate bindings (IN nix-shell)
nix-shell
just gen-swift
exit

# Step 2: Test bindings (OUTSIDE nix-shell)
just test-swift
```

## Why This Workflow?

1. **uniffi-bindgen** is only available in nix-shell (installed via pip)
2. **System Swift SDK** (v6.2.1) is incompatible with nix SDK (v11.3)
3. **Solution**: Build Rust library WITHOUT GLib (no nix dependencies), generate bindings in nix-shell, test outside nix-shell

## What Changed?

### GLib is Now Optional

The `glib` feature is now optional (enabled by default):

```toml
[features]
default = ["glib"]
glib = ["glib-sys"]
```

### Swift Builds Without GLib

When generating Swift bindings, the library is built with `--no-default-features`, which:
- ✅ Removes nix GLib dependency
- ✅ Library works with system Swift
- ❌ `process_part2_glib` function unavailable (8 of 9 implementations still work)

## Detailed Steps

### In Nix-Shell (Generation)

```bash
$ nix-shell
[nix-shell]$ just gen-swift
Building library without GLib (for Swift)...
    Finished `release` profile [optimized] target(s) in 0.87s
Generating Swift bindings...
✓ Swift bindings generated in bindings/swift/
Note: GLib-based implementation (process_part2_glib) is unavailable
```

This creates:
- `bindings/swift/aoc_ffi_day01.swift` - Swift bindings
- `bindings/swift/aoc_ffi_day01FFI.h` - C header
- `bindings/swift/aoc_ffi_day01FFI.modulemap` - Module map
- `bindings/swift/libaoc_ffi_day01.dylib` - Rust library (NO nix dependencies!)

### Outside Nix-Shell (Testing)

```bash
$ exit  # Exit nix-shell
$ just test-swift
Testing Swift bindings...
Note: GLib-based implementation is excluded from Swift tests

Testing Advent of Code 2024 Day 1 - Swift Bindings

============================================================

Part 1: Sum of absolute differences
------------------------------------------------------------
  Rust (native):      11
  C (manual FFI):     11
  libc crate:         11

  ✓ All Part 1 implementations return 11

Part 2: Similarity score
------------------------------------------------------------
  Rust (native)             31
  C-style                   31
  Binary search             31
  FreqMap (simulated)       31
  Real C (count)            31
  Real C (freqmap)          31
  libc crate                31
  uthash                    31

  ✓ All Part 2 implementations return 31

============================================================
✓ All tests passed!

✓ Swift tests completed
```

## Troubleshooting

### Error: "Swift bindings not found"
Run `just gen-swift` in nix-shell first.

### Error: "You are currently IN nix-shell"
Exit nix-shell before running `just test-swift`.

### Segmentation Fault
The dylib in `bindings/swift/` has nix dependencies. Regenerate bindings:
```bash
nix-shell
just gen-swift
exit
just test-swift
```

### Other Languages Work Differently

- **Python**: Generate AND test in nix-shell (`just test-python`)
- **Kotlin**: Generate AND test in nix-shell (`just test-kotlin`)
- **Swift**: Generate in nix-shell, test OUTSIDE nix-shell

## Library Dependencies

```bash
# With GLib (Python/Kotlin)
$ otool -L target/release/libaoc_ffi_day01.dylib
/nix/store/.../libglib-2.0.0.dylib    # ← nix dependency
/usr/lib/libSystem.B.dylib

# Without GLib (Swift)
$ otool -L target/release/libaoc_ffi_day01.dylib
/usr/lib/libiconv.2.dylib             # ← system library
/usr/lib/libSystem.B.dylib            # ← system library
```

## Summary

| Language | Build Features | Generate Where | Test Where |
|----------|---------------|----------------|------------|
| Python   | `default` (with GLib) | nix-shell | nix-shell |
| Kotlin   | `default` (with GLib) | nix-shell | nix-shell |
| Swift    | `--no-default-features` (no GLib) | nix-shell | **outside nix-shell** |
