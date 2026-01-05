//! 2024 Day 1: Historian Hysteria
//! 
//! see the README for more info

use std::ffi::c_void;
use std::os::raw::c_int;

// FFI declaration for libc's qsort
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

fn process(input: &str) -> Result<i32, String> { 
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = unzip(input);

    // critical: sort both lists
    // left.sort();
    // right.sort();

    // use C qsort instead of Rust's stable_sort (timsort)
    c_qsort(&mut left);
    c_qsort(&mut right);

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

fn main() {
    println!("2024 Day 1");

    assert_eq!(process("3 7").unwrap(), 4);
    assert_eq!(process("9 3").unwrap(), 6);

    println!("Part 1: {}", process("3   4
4   3
2   5
1   3
3   9
3   3").unwrap())
}
