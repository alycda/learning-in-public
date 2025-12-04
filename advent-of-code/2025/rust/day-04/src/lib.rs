use std::str::FromStr;
use aoc_ornaments::spatial::{Grid, Position};

pub fn part1(grid: Grid::<char>) -> Vec<Position> {
    // let grid = Grid::<char>::from_str(input).unwrap();
    let mut accessible = vec![];

    // dbg!(grid);

    grid.walk(|pos| {
        if grid.get_at_unbounded(pos) == '@' {
            let neighbors = grid
                .get_all_neighbors(pos)
                .iter().filter(|(pos, c)|{ *c == '@' }).count();

            // dbg!(neighbors);

            if neighbors < 4 {
                accessible.push(pos);
            }
        }
    });

    accessible
}

pub fn part2(mut grid: Grid::<char>) -> usize {
    let mut removable = part1(grid);
    let mut count = removable.len();

    while removable.len() > 0 {
        removable.iter().for_each(|pos| {
            grid.set_at_unbounded(*pos, '.');

            removable = part1(grid);
            count += removable.len();
        });
    }

    count
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
        let mut grid = Grid::<char>::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(part1(grid).len(), 13);
    }

    #[test]
    fn test_part2() {
        let mut grid = Grid::<char>::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(part2(grid), 43);
    }
}