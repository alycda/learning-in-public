use std::str::FromStr;

use aoc_ornaments::spatial::Grid;
use day_04::part1;

fn main() {
    #[cfg(feature = "ci")]
    let file = day_04::SAMPLE_INPUT;

    #[cfg(not(feature = "ci"))]
    let file = include_str!("../../input.txt");

    let grid = Grid::<char>::from_str(file).unwrap();
    let result = part1(&grid);
    println!("{}", result.len());
}