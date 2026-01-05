//! 2024 Day 1: Historian Hysteria
//! 
//! see the README for more info

use day_01::{process_rust, process_c};

fn main() {
    println!("2024 Day 1");

    assert_eq!(process_rust("3 7").unwrap(), process_c("3 7").unwrap());
    assert_eq!(process_c("9 3").unwrap(), process_rust("9 3").unwrap());

    println!("Part 1: {}/{}", process_rust(day_01::SAMPLE_INPUT).unwrap(), process_c(day_01::SAMPLE_INPUT).unwrap())
}
