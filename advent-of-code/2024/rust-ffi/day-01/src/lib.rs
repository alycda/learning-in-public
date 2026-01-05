use std::ffi::c_void;
use std::os::raw::c_int;

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
pub fn process(input: &str) -> Result<i32, String> {
    solve::<NativeSort>(input)
}

// Shorthand function for C qsort
pub fn process_c(input: &str) -> Result<i32, String> {
    solve::<CSort>(input)
}