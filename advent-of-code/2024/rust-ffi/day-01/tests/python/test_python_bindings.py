#!/usr/bin/env python3
"""
Test Python bindings for the Advent of Code 2024 Day 1 Rust library.

Run this after generating bindings with bindings_gen.py
"""

import sys
from pathlib import Path

# Add bindings directory to path
bindings_dir = Path(__file__).parent / "bindings" / "python"
sys.path.insert(0, str(bindings_dir))

try:
    import aoc_ffi_day01
except ImportError as e:
    print(f"Error importing bindings: {e}")
    print("\nPlease generate bindings first:")
    print("  python3 bindings_gen.py")
    print("\nOr install uniffi-bindgen and run:")
    print("  pip install uniffi-bindgen==0.28.3")
    print("  uniffi-bindgen generate src/aoc_ffi_day01.udl --lib-file target/release/libaoc_ffi_day01.dylib --language python --out-dir bindings/python")
    sys.exit(1)

SAMPLE_INPUT = """3   4
4   3
2   5
1   3
3   9
3   3"""

def main():
    print("Testing Advent of Code 2024 Day 1 - Python Bindings\n")
    print("=" * 60)

    # Test Part 1 implementations
    print("\nPart 1: Sum of absolute differences")
    print("-" * 60)

    try:
        part1_rust = aoc_ffi_day01.uniffi_process_part1(SAMPLE_INPUT)
        print(f"  Rust (native):      {part1_rust}")

        part1_c = aoc_ffi_day01.uniffi_process_part1_c(SAMPLE_INPUT)
        print(f"  C (manual FFI):     {part1_c}")

        part1_libc = aoc_ffi_day01.uniffi_process_part1_libc(SAMPLE_INPUT)
        print(f"  libc crate:         {part1_libc}")

        assert part1_rust == part1_c == part1_libc == 11, "Part 1 results don't match!"
        print("\n  ✓ All Part 1 implementations return 11")

    except aoc_ffi_day01.AocError as e:
        print(f"  ✗ Error: {e}")
        return 1

    # Test Part 2 implementations
    print("\nPart 2: Similarity score")
    print("-" * 60)

    implementations = [
        ("Rust (native)", aoc_ffi_day01.uniffi_process_part2),
        ("C-style", aoc_ffi_day01.uniffi_process_part2_c),
        ("Binary search", aoc_ffi_day01.uniffi_process_part2_bsearch),
        ("FreqMap (simulated)", aoc_ffi_day01.uniffi_process_part2_freqmap),
        ("Real C (count)", aoc_ffi_day01.uniffi_process_part2_glibc_count),
        ("Real C (freqmap)", aoc_ffi_day01.uniffi_process_part2_glibc_freqmap),
        ("libc crate", aoc_ffi_day01.uniffi_process_part2_libc),
        ("GLib hash table", aoc_ffi_day01.uniffi_process_part2_glib),
        ("uthash", aoc_ffi_day01.uniffi_process_part2_uthash),
    ]

    results = []
    for name, func in implementations:
        try:
            result = func(SAMPLE_INPUT)
            results.append(result)
            print(f"  {name:25s} {result}")
        except aoc_ffi_day01.AocError as e:
            print(f"  {name:25s} Error: {e}")
            return 1

    # Verify all results match
    expected = 31
    if all(r == expected for r in results):
        print(f"\n  ✓ All Part 2 implementations return {expected}")
    else:
        print(f"\n  ✗ Results don't match! Expected {expected}, got {set(results)}")
        return 1

    # Test error handling
    print("\nError Handling Test")
    print("-" * 60)
    try:
        # This should work fine
        result = aoc_ffi_day01.uniffi_process_part1("1 2")
        print(f"  Valid input '1 2' → {result}")
    except aoc_ffi_day01.AocError as e:
        print(f"  Unexpected error: {e}")

    print("\n" + "=" * 60)
    print("✓ All tests passed!\n")
    return 0

if __name__ == "__main__":
    sys.exit(main())
