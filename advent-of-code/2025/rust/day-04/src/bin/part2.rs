use std::str::FromStr;

use aoc_ornaments::spatial::Grid;
use day_04::part2;

fn main() {
    let file = include_str!("../../input.txt");

    let grid = Grid::<char>::from_str(file).unwrap();
    let result = part2(grid);
    println!("{}", result);
}