//! 2024 Day 1: Historian Hysteria
//!
//! see the README for more info

use day_01::{process, process_c, SAMPLE_INPUT};

fn main() {
    println!("2024 Day 1");

    assert_eq!(process("3 7").unwrap(), process_c("3 7").unwrap());
    assert_eq!(process_c("9 3").unwrap(), process("9 3").unwrap());

    println!("Part 1: {}/{}", process(SAMPLE_INPUT).unwrap(), process_c(SAMPLE_INPUT).unwrap())
}
