# Swift Bindings Limitations on macOS with Nix

## The Problem

Swift bindings cannot be tested in this project due to a fundamental incompatibility between:
1. **Nix-provided dependencies** (GLib 2.84.3 from nix store)
2. **System Swift compiler** (v6.2.1) with system SDK

## Why This Happens

The Rust library (`libaoc_ffi_day01.dylib`) links against GLib from the nix store:
```
/nix/store/papv53lfi9dy6aa2h9ni4ksgh7b5hnsv-glib-2.84.3/lib/libglib-2.0.0.dylib
```

### Scenario 1: Testing Inside Nix-Shell
- ✅ GLib dependencies are available
- ❌ Swift SDK mismatch: nix SDK (v11.3, Swift 5.4) vs system Swift (v6.2.1)
- **Result**: Swift compilation fails with SDK incompatibility errors

### Scenario 2: Testing Outside Nix-Shell
- ✅ System Swift SDK works correctly
- ❌ GLib libraries not found (nix store paths unavailable)
- **Result**: Runtime segmentation fault when loading dylib

## What Works

- ✅ **Python bindings**: Work perfectly (tested in nix-shell)
- ✅ **Kotlin bindings**: Work perfectly (tested in nix-shell with JNA)
- ✅ **Swift bindings generation**: UniFFI successfully generates Swift code
- ❌ **Swift bindings testing**: Cannot run due to dependency conflicts

## Potential Solutions

### Option 1: Remove GLib Dependency
Make GLib optional via Cargo features, but this loses the `process_part2_glib` implementation.

```toml
[dependencies]
glib-sys = { version = "0.20", optional = true }

[features]
default = []
with-glib = ["glib-sys"]
```

### Option 2: Use System GLib (via Homebrew)
Install GLib system-wide:
```bash
brew install glib
```

Then configure the build to use system GLib instead of nix GLib. However, this defeats the purpose of using nix for reproducible builds.

### Option 3: Accept the Limitation
Swift bindings work conceptually but cannot be tested in this nix-based environment. This is a known limitation when mixing nix with platform-specific toolchains.

## Files Provided

- `bindings/swift/aoc_ffi_day01.swift`: Generated Swift bindings ✅
- `bindings/swift/aoc_ffi_day01FFI.h`: C header for FFI ✅
- `test_swift_bindings.swift`: Swift test script ✅
- `test_swift_bindings.sh`: Shell script to compile and run tests ⚠️ (will segfault)

## Recommendation

For a production UniFFI project targeting Swift on macOS:
1. **Don't use nix** for C dependencies like GLib
2. Use Homebrew or system libraries
3. OR: Make all C dependencies optional via Cargo features
4. OR: Build everything (including Swift) inside a consistent nix environment with compatible Swift toolchain

## Conclusion

This project successfully demonstrates UniFFI working with Python and Kotlin. The Swift limitation is environmental, not conceptual - the bindings are correctly generated and would work in a non-nix environment or with properly configured system dependencies.
