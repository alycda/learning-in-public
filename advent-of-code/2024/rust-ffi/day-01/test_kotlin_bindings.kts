#!/usr/bin/env kotlin

/**
 * Test Kotlin bindings for the Advent of Code 2024 Day 1 Rust library.
 *
 * Run this after generating bindings with: just gen-kotlin
 *
 * To run:
 *   cd bindings/kotlin
 *   kotlinc -script ../../test_kotlin_bindings.kts -classpath .
 */

@file:CompilerOptions("-jvm-target", "1.8")

import uniffi.aoc_ffi_day01.uniffiProcessPart1
import uniffi.aoc_ffi_day01.uniffiProcessPart1C
import uniffi.aoc_ffi_day01.uniffiProcessPart1Libc
import uniffi.aoc_ffi_day01.uniffiProcessPart2
import uniffi.aoc_ffi_day01.uniffiProcessPart2C
import uniffi.aoc_ffi_day01.uniffiProcessPart2Bsearch
import uniffi.aoc_ffi_day01.uniffiProcessPart2Freqmap
import uniffi.aoc_ffi_day01.uniffiProcessPart2GlibcCount
import uniffi.aoc_ffi_day01.uniffiProcessPart2GlibcFreqmap
import uniffi.aoc_ffi_day01.uniffiProcessPart2Libc
import uniffi.aoc_ffi_day01.uniffiProcessPart2Glib
import uniffi.aoc_ffi_day01.uniffiProcessPart2Uthash
import uniffi.aoc_ffi_day01.AocException

val SAMPLE_INPUT = """3   4
4   3
2   5
1   3
3   9
3   3"""

fun main() {
    println("Testing Advent of Code 2024 Day 1 - Kotlin Bindings\n")
    println("=" .repeat(60))

    // Test Part 1 implementations
    println("\nPart 1: Sum of absolute differences")
    println("-".repeat(60))

    try {
        val part1Rust = uniffiProcessPart1(SAMPLE_INPUT)
        println("  Rust (native):      $part1Rust")

        val part1C = uniffiProcessPart1C(SAMPLE_INPUT)
        println("  C (manual FFI):     $part1C")

        val part1Libc = uniffiProcessPart1Libc(SAMPLE_INPUT)
        println("  libc crate:         $part1Libc")

        assert(part1Rust == 11 && part1C == 11 && part1Libc == 11) {
            "Part 1 results don't match!"
        }
        println("\n  ✓ All Part 1 implementations return 11")

    } catch (e: AocException) {
        println("  ✗ Error: ${e.message}")
        return
    }

    // Test Part 2 implementations
    println("\nPart 2: Similarity score")
    println("-".repeat(60))

    val implementations = listOf(
        "Rust (native)" to ::uniffiProcessPart2,
        "C-style" to ::uniffiProcessPart2C,
        "Binary search" to ::uniffiProcessPart2Bsearch,
        "FreqMap (simulated)" to ::uniffiProcessPart2Freqmap,
        "Real C (count)" to ::uniffiProcessPart2GlibcCount,
        "Real C (freqmap)" to ::uniffiProcessPart2GlibcFreqmap,
        "libc crate" to ::uniffiProcessPart2Libc,
        "GLib hash table" to ::uniffiProcessPart2Glib,
        "uthash" to ::uniffiProcessPart2Uthash
    )

    val results = mutableListOf<Int>()
    for ((name, func) in implementations) {
        try {
            val result = func(SAMPLE_INPUT)
            results.add(result)
            println("  %-25s %d".format(name, result))
        } catch (e: AocException) {
            println("  %-25s Error: ${e.message}".format(name))
            return
        }
    }

    // Verify all results match
    val expected = 31
    if (results.all { it == expected }) {
        println("\n  ✓ All Part 2 implementations return $expected")
    } else {
        println("\n  ✗ Results don't match! Expected $expected, got ${results.toSet()}")
        return
    }

    println("\n" + "=".repeat(60))
    println("✓ All tests passed!\n")
}

main()
