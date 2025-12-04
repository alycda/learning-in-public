use std::str::FromStr;
use aoc_ornaments::spatial::Grid;

pub fn part1(input: &str) -> u32 {
    let grid = Grid::<char>::from_str(input).unwrap();
    let mut accessible = 0;

    // dbg!(grid);

    grid.walk(|pos| {
        if grid.get_at_unbounded(pos) == '@' {
            let neighbors = grid
                .get_all_neighbors(pos)
                .iter().filter(|(pos, c)|{ *c == '@' }).count();

            // dbg!(neighbors);

            if neighbors < 4 {
                accessible += 1
            }
        }
    });

    accessible
}

pub const SAMPLE_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT), 13);
    }
}