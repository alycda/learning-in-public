use std::ffi::c_void;
use std::os::raw::c_int;
use libc::{qsort as libc_qsort, bsearch as libc_bsearch};

// Trait for different sorting strategies
pub trait Sorter {
    fn sort(vec: &mut Vec<i32>);
}

// Marker type for native Rust sorting
pub struct NativeSort;

impl Sorter for NativeSort {
    fn sort(vec: &mut Vec<i32>) {
        vec.sort();
    }
}

// Marker type for C qsort
pub struct CSort;

impl Sorter for CSort {
    fn sort(vec: &mut Vec<i32>) {
        c_qsort(vec);
    }
}

pub const SAMPLE_INPUT: &'static str = "3   4
4   3
2   5
1   3
3   9
3   3";

// FFI declaration for libc's qsort and bsearch
unsafe extern "C" {
    // https://www.tutorialspoint.com/c_standard_library/c_function_qsort.htm
    fn qsort(
        // the pointer to the first element of the array to be sorted
        base: *mut c_void,
        // number of elements in the array
        num: usize,
        // size of each element in the array
        size: usize,
        // a function pointer to a 2-element comparison function
        compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int
    );

    // https://www.tutorialspoint.com/c_standard_library/c_function_bsearch.htm
    fn bsearch(
        // pointer to the key to search for
        key: *const c_void,
        // pointer to the first element of the sorted array
        base: *const c_void,
        // number of elements in the array
        num: usize,
        // size of each element
        size: usize,
        // comparison function
        compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int
    ) -> *mut c_void;
}

// Comparison function for qsort (ascending order)
// Must return: negative if a < b, zero if a == b, positive if a > b
unsafe extern "C" fn compare_i32(a: *const c_void, b: *const c_void) -> c_int {
    // SAFETY: qsort guarantees valid pointers to i32 elements
    unsafe {
        let a = *(a as *const i32);
        let b = *(b as *const i32);
        a - b  // Simple subtraction works for i32 comparisons vs `a.cmp(&b) as c_int`
    }
}

// Wrapper that calls C's qsort on a Rust Vec<i32>
// SAFETY: Vec's memory layout is compatible with C arrays (contiguous, aligned)
// qsort won't resize/reallocate, just reorders elements in place
fn c_qsort(vec: &mut Vec<i32>) {
    unsafe {
        qsort(
            // pointer to the first element in the array
            vec.as_mut_ptr() as *mut c_void,
            // number of elements in the array
            vec.len(),
            // size of each element in the array
            std::mem::size_of::<i32>(),
            // comparison function
            compare_i32
        );
    }
}

/// transpose 2 colums of numbers
fn unzip(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let nums = line
                // from columns
                .split_whitespace()
                // parse number
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            (nums[0], nums[1])
        })
        .unzip()
}

// Generic process function that works with any Sorter implementation
pub fn solve<S: Sorter>(input: &str) -> Result<i32, String> {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = unzip(input);

    // Sort using the strategy provided by type parameter S
    S::sort(&mut left);
    S::sort(&mut right);

    Ok(left
        .iter()
        // for each element
        .zip(right.iter())
        // get the absolute difference
        .map(|(l, r)| (l-r).abs())
        // and sum
        .sum::<i32>()
    )
}

// Shorthand function for native Rust sorting
pub fn process_part1(input: &str) -> Result<i32, String> {
    solve::<NativeSort>(input)
}

// Shorthand function for C qsort
pub fn process_part1_c(input: &str) -> Result<i32, String> {
    solve::<CSort>(input)
}

// Manual C-style count function (simulating a C implementation)
// In a real FFI scenario, this would be in a .c file and linked
unsafe fn c_count_occurrences(arr: *const i32, len: usize, target: i32) -> usize {
    let mut count = 0;
    // SAFETY: Caller guarantees arr points to len valid i32 elements
    unsafe {
        for i in 0..len {
            if *arr.add(i) == target {
                count += 1;
            }
        }
    }
    count
}

// Wrapper that safely calls the C-style count function
fn count_with_c(vec: &[i32], target: i32) -> usize {
    unsafe {
        c_count_occurrences(vec.as_ptr(), vec.len(), target)
    }
}

pub fn process_part2(input: &str) -> Result<i32, String> {
    let (left, right): (Vec<i32>, Vec<i32>) = unzip(input);

    Ok(left
        .iter()
        .map(|n| n * right.iter().filter(|&x| x==n).count() as i32)
        .sum())
}

// Part 2 using C-style counting via FFI
pub fn process_part2_c(input: &str) -> Result<i32, String> {
    let (left, right): (Vec<i32>, Vec<i32>) = unzip(input);

    Ok(left
        .iter()
        .map(|n| n * count_with_c(&right, *n) as i32)
        .sum())
}

// Count occurrences using C's bsearch + manual counting
// More efficient than linear scan: O(log n) search + O(k) count where k = occurrences
unsafe fn c_bsearch_count(sorted_arr: *const i32, len: usize, target: i32) -> usize {
    if len == 0 {
        return 0;
    }

    // SAFETY: Caller guarantees sorted_arr points to len valid i32 elements
    unsafe {
        // Use bsearch to find ANY occurrence of target
        let found = bsearch(
            &target as *const i32 as *const c_void,
            sorted_arr as *const c_void,
            len,
            std::mem::size_of::<i32>(),
            compare_i32
        );

        // bsearch returns null if not found
        if found.is_null() {
            return 0;
        }

        let found_ptr = found as *const i32;

        // Count backwards to find first occurrence
        let mut first = found_ptr;
        while first > sorted_arr && *first.offset(-1) == target {
            first = first.offset(-1);
        }

        // Count forwards to find last occurrence
        let end_ptr = sorted_arr.add(len);
        let mut last = found_ptr;
        while last < end_ptr.offset(-1) && *last.offset(1) == target {
            last = last.offset(1);
        }

        // Calculate count: (last - first) + 1
        (last.offset_from(first) + 1) as usize
    }
}

// Wrapper for bsearch-based counting
fn count_with_bsearch(sorted_vec: &[i32], target: i32) -> usize {
    unsafe {
        c_bsearch_count(sorted_vec.as_ptr(), sorted_vec.len(), target)
    }
}

// Part 2 using bsearch + counting
// This version sorts the right array first, then uses binary search
pub fn process_part2_bsearch(input: &str) -> Result<i32, String> {
    let (left, mut right): (Vec<i32>, Vec<i32>) = unzip(input);

    // Sort right array for binary search
    c_qsort(&mut right);

    Ok(left
        .iter()
        .map(|n| n * count_with_bsearch(&right, *n) as i32)
        .sum())
}

// C-style frequency map using a simple array
// This simulates what you'd do in C with malloc/calloc
#[repr(C)]
struct FrequencyMap {
    // We'll use a simple array approach: map[value] = count
    // Assumes values are in a reasonable range (e.g., 0-100000)
    min_val: i32,
    max_val: i32,
    counts: *mut i32,  // Heap-allocated array of counts
}

impl FrequencyMap {
    // Build frequency map from array (C-style malloc + iteration)
    unsafe fn from_array(arr: *const i32, len: usize) -> Self {
        if len == 0 {
            return FrequencyMap {
                min_val: 0,
                max_val: 0,
                counts: std::ptr::null_mut(),
            };
        }

        // Find min and max to determine array size needed
        unsafe {
            let mut min_val = *arr;
            let mut max_val = *arr;
            for i in 0..len {
                let val = *arr.add(i);
                if val < min_val { min_val = val; }
                if val > max_val { max_val = val; }
            }

            let range = (max_val - min_val + 1) as usize;

            // Allocate zeroed memory (like calloc in C)
            let layout = std::alloc::Layout::array::<i32>(range).unwrap();
            let counts = std::alloc::alloc_zeroed(layout) as *mut i32;

            if counts.is_null() {
                panic!("Failed to allocate memory");
            }

            // Count frequencies
            for i in 0..len {
                let val = *arr.add(i);
                let idx = (val - min_val) as usize;
                *counts.add(idx) += 1;
            }

            FrequencyMap {
                min_val,
                max_val,
                counts,
            }
        }
    }

    // Lookup count for a value
    unsafe fn get(&self, value: i32) -> i32 {
        if self.counts.is_null() || value < self.min_val || value > self.max_val {
            return 0;
        }

        unsafe {
            let idx = (value - self.min_val) as usize;
            *self.counts.add(idx)
        }
    }

    // Free allocated memory (like free() in C)
    unsafe fn free(&mut self) {
        if !self.counts.is_null() {
            unsafe {
                let range = (self.max_val - self.min_val + 1) as usize;
                let layout = std::alloc::Layout::array::<i32>(range).unwrap();
                std::alloc::dealloc(self.counts as *mut u8, layout);
                self.counts = std::ptr::null_mut();
            }
        }
    }
}

// Wrapper that uses C-style frequency map
fn count_with_freqmap(freq_map: &FrequencyMap, target: i32) -> i32 {
    unsafe { freq_map.get(target) }
}

// Part 2 using C-style frequency map
// Most efficient: O(n) to build map, O(1) lookups
pub fn process_part2_freqmap(input: &str) -> Result<i32, String> {
    let (left, right): (Vec<i32>, Vec<i32>) = unzip(input);

    // Build frequency map from right array (simulates C malloc + counting)
    let mut freq_map = unsafe {
        FrequencyMap::from_array(right.as_ptr(), right.len())
    };

    let result = Ok(left
        .iter()
        .map(|n| n * count_with_freqmap(&freq_map, *n))
        .sum());

    // Clean up (simulates C free())
    unsafe { freq_map.free(); }

    result
}

// ============================================================================
// Real C FFI - linking to actual compiled C code from aoc_ffi.c
// ============================================================================

// C FreqMap struct representation (must match C definition)
#[repr(C)]
struct CFreqMap {
    min_val: i32,
    max_val: i32,
    counts: *mut i32,
}

// External C functions from our compiled library
unsafe extern "C" {
    // size_t count_occurrences(const int32_t *arr, size_t len, int32_t target);
    fn count_occurrences(arr: *const i32, len: usize, target: i32) -> usize;

    // FreqMap* freqmap_build(const int32_t *arr, size_t len);
    fn freqmap_build(arr: *const i32, len: usize) -> *mut CFreqMap;

    // int32_t freqmap_get(const FreqMap *map, int32_t value);
    fn freqmap_get(map: *const CFreqMap, value: i32) -> i32;

    // void freqmap_free(FreqMap *map);
    fn freqmap_free(map: *mut CFreqMap);
}

// Part 2 using real C counting function
pub fn process_part2_glibc_count(input: &str) -> Result<i32, String> {
    let (left, right): (Vec<i32>, Vec<i32>) = unzip(input);

    Ok(left
        .iter()
        .map(|n| {
            let count = unsafe {
                count_occurrences(right.as_ptr(), right.len(), *n)
            };
            n * count as i32
        })
        .sum())
}

// Part 2 using real C frequency map
pub fn process_part2_glibc_freqmap(input: &str) -> Result<i32, String> {
    let (left, right): (Vec<i32>, Vec<i32>) = unzip(input);

    // Build frequency map using C
    let freq_map = unsafe {
        freqmap_build(right.as_ptr(), right.len())
    };

    if freq_map.is_null() {
        return Err("Failed to build frequency map".to_string());
    }

    let result = left
        .iter()
        .map(|n| {
            let count = unsafe { freqmap_get(freq_map, *n) };
            n * count
        })
        .sum();

    // Free the C-allocated map
    unsafe { freqmap_free(freq_map); }

    Ok(result)
}

// ============================================================================
// Using libc crate - pre-made FFI bindings to C standard library
// ============================================================================

// Wrapper that uses libc crate's qsort instead of our manual declaration
fn libc_crate_qsort(vec: &mut Vec<i32>) {
    unsafe {
        libc_qsort(
            vec.as_mut_ptr() as *mut c_void,
            vec.len(),
            std::mem::size_of::<i32>(),
            Some(compare_i32)  // libc crate uses Option<fn>
        );
    }
}

// Part 1 using libc crate's qsort
pub fn process_part1_libc(input: &str) -> Result<i32, String> {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = unzip(input);

    // Sort using libc crate's qsort
    libc_crate_qsort(&mut left);
    libc_crate_qsort(&mut right);

    Ok(left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l-r).abs())
        .sum::<i32>()
    )
}

// Binary search using libc crate's bsearch
unsafe fn libc_crate_bsearch_count(sorted_arr: *const i32, len: usize, target: i32) -> usize {
    if len == 0 {
        return 0;
    }

    unsafe {
        let found = libc_bsearch(
            &target as *const i32 as *const c_void,
            sorted_arr as *const c_void,
            len,
            std::mem::size_of::<i32>(),
            Some(compare_i32)  // libc crate uses Option<fn>
        );

        if found.is_null() {
            return 0;
        }

        let found_ptr = found as *const i32;

        // Count backwards to find first occurrence
        let mut first = found_ptr;
        while first > sorted_arr && *first.offset(-1) == target {
            first = first.offset(-1);
        }

        // Count forwards to find last occurrence
        let end_ptr = sorted_arr.add(len);
        let mut last = found_ptr;
        while last < end_ptr.offset(-1) && *last.offset(1) == target {
            last = last.offset(1);
        }

        (last.offset_from(first) + 1) as usize
    }
}

// Part 2 using libc crate's bsearch
pub fn process_part2_libc(input: &str) -> Result<i32, String> {
    let (left, mut right): (Vec<i32>, Vec<i32>) = unzip(input);

    // Sort with libc's qsort
    libc_crate_qsort(&mut right);

    Ok(left
        .iter()
        .map(|n| {
            let count = unsafe {
                libc_crate_bsearch_count(right.as_ptr(), right.len(), *n)
            };
            n * count as i32
        })
        .sum())
}

// ============================================================================
// Using glib-sys - GLib hash table for frequency counting
// ============================================================================

use glib_sys::{
    g_hash_table_new, g_hash_table_destroy, g_hash_table_insert,
    g_hash_table_lookup, g_direct_hash, g_direct_equal, gpointer
};

// Build frequency map using GLib's GHashTable
// GHashTable is GLib's hash map implementation (similar to HashMap in Rust)
unsafe fn glib_build_freq_map(arr: &[i32]) -> *mut glib_sys::GHashTable {
    unsafe {
        // Create hash table with direct hash (for integer keys stored as pointers)
        let table = g_hash_table_new(
            Some(g_direct_hash),    // hash function for integer keys
            Some(g_direct_equal)     // equality function for integer keys
        );

        // Count frequencies
        for &value in arr {
            // GLib stores keys/values as void pointers (gpointer)
            let key = value as gpointer;

            // Look up current count (stored as pointer)
            let current = g_hash_table_lookup(table, key);
            let count = if current.is_null() {
                0
            } else {
                current as isize
            };

            // Store incremented count
            let new_count = (count + 1) as gpointer;
            g_hash_table_insert(table, key, new_count);
        }

        table
    }
}

// Lookup frequency in GHashTable
unsafe fn glib_get_freq(table: *mut glib_sys::GHashTable, value: i32) -> i32 {
    unsafe {
        let key = value as gpointer;
        let result = g_hash_table_lookup(table, key);

        if result.is_null() {
            0
        } else {
            result as i32
        }
    }
}

// Part 2 using GLib's hash table
pub fn process_part2_glib(input: &str) -> Result<i32, String> {
    let (left, right): (Vec<i32>, Vec<i32>) = unzip(input);

    unsafe {
        // Build frequency map using GLib
        let freq_table = glib_build_freq_map(&right);

        let result = left
            .iter()
            .map(|n| {
                let count = glib_get_freq(freq_table, *n);
                n * count
            })
            .sum();

        // Clean up GLib hash table
        g_hash_table_destroy(freq_table);

        Ok(result)
    }
}