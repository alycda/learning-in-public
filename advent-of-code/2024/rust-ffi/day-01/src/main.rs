//! 2024 Day 1: Historian Hysteria
//!
//! see the README for more info

use day_01::{process_part1, process_part2, process_part1_c, process_part2_c, process_part2_bsearch, SAMPLE_INPUT};

fn main() {
    println!("2024 Day 1");

    assert_eq!(process_part1("3 7").unwrap(), process_part1_c("3 7").unwrap());
    assert_eq!(process_part1_c("9 3").unwrap(), process_part1("9 3").unwrap());

    println!("Part 1: {}/{}", process_part1(SAMPLE_INPUT).unwrap(), process_part1_c(SAMPLE_INPUT).unwrap());

    let part2 = process_part2(SAMPLE_INPUT).unwrap();
    let part2_c = process_part2_c(SAMPLE_INPUT).unwrap();
    let part2_bsearch = process_part2_bsearch(SAMPLE_INPUT).unwrap();
    assert_eq!(part2, 31);
    assert_eq!(part2_c, 31);
    assert_eq!(part2_bsearch, 31);

    println!("Part 2: {}/{}/{}", part2, part2_c, part2_bsearch)
}
