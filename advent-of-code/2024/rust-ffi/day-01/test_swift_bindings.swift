/**
 * Test Swift bindings for the Advent of Code 2024 Day 1 Rust library.
 *
 * Run this after generating bindings with: just gen-swift
 *
 * To compile and run:
 *   cd bindings/swift
 *   swiftc -o test_swift ../../test_swift_bindings.swift aoc_ffi_day01.swift -L . -laoc_ffi_day01
 *   ./test_swift
 */

import Foundation

@main
struct TestRunner {
    static func main() {
        let SAMPLE_INPUT = """
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
        """

        print("Testing Advent of Code 2024 Day 1 - Swift Bindings\n")
        print(String(repeating: "=", count: 60))

        // Test Part 1 implementations
        print("\nPart 1: Sum of absolute differences")
        print(String(repeating: "-", count: 60))

        do {
            let part1Rust = try uniffiProcessPart1(input: SAMPLE_INPUT)
            print("  Rust (native):      \(part1Rust)")

            let part1C = try uniffiProcessPart1C(input: SAMPLE_INPUT)
            print("  C (manual FFI):     \(part1C)")

            let part1Libc = try uniffiProcessPart1Libc(input: SAMPLE_INPUT)
            print("  libc crate:         \(part1Libc)")

            assert(part1Rust == 11 && part1C == 11 && part1Libc == 11,
                   "Part 1 results don't match!")
            print("\n  ✓ All Part 1 implementations return 11")

        } catch {
            print("  ✗ Error: \(error)")
            exit(1)
        }

        // Test Part 2 implementations
        print("\nPart 2: Similarity score")
        print(String(repeating: "-", count: 60))

        // Note: GLib implementation is excluded (requires nix GLib dependency)
        let implementations: [(String, (String) throws -> Int32)] = [
            ("Rust (native)", uniffiProcessPart2),
            ("C-style", uniffiProcessPart2C),
            ("Binary search", uniffiProcessPart2Bsearch),
            ("FreqMap (simulated)", uniffiProcessPart2Freqmap),
            ("Real C (count)", uniffiProcessPart2GlibcCount),
            ("Real C (freqmap)", uniffiProcessPart2GlibcFreqmap),
            ("libc crate", uniffiProcessPart2Libc),
            // ("GLib hash table", uniffiProcessPart2Glib),  // Unavailable without glib feature
            ("uthash", uniffiProcessPart2Uthash)
        ]

        var results: [Int32] = []
        for (name, function) in implementations {
            do {
                let result = try function(SAMPLE_INPUT)
                results.append(result)
                // Use Swift string interpolation instead of C-style format strings
                let paddedName = name.padding(toLength: 25, withPad: " ", startingAt: 0)
                print("  \(paddedName) \(result)")
            } catch {
                let paddedName = name.padding(toLength: 25, withPad: " ", startingAt: 0)
                print("  \(paddedName) Error: \(error)")
                exit(1)
            }
        }

        // Verify all results match
        let expected: Int32 = 31
        if results.allSatisfy({ $0 == expected }) {
            print("\n  ✓ All Part 2 implementations return \(expected)")
        } else {
            print("\n  ✗ Results don't match! Expected \(expected), got \(Set(results))")
            exit(1)
        }

        print("\n" + String(repeating: "=", count: 60))
        print("✓ All tests passed!\n")
    }
}
