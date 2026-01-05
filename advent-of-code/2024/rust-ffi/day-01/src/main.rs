//! 2024 Day 1: Historian Hysteria
//! 
//! see the README for more info

use day_01::{process, NativeSort, CSort};

fn main() {
    println!("2024 Day 1");

    // TODO: alias shorthand fns

    assert_eq!(process::<NativeSort>("3 7").unwrap(), process::<CSort>("3 7").unwrap());
    assert_eq!(process::<CSort>("9 3").unwrap(), process::<NativeSort>("9 3").unwrap());

    println!("Part 1: {}/{}", process::<NativeSort>(day_01::SAMPLE_INPUT).unwrap(), process::<CSort>(day_01::SAMPLE_INPUT).unwrap())
}
