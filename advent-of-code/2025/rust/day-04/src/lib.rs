use aoc_ornaments::spatial::{Grid, Position};

pub fn part1(grid: &Grid<char>) -> Vec<Position> {  // Take reference, not ownership
    let mut accessible = vec![];

    grid.walk(|pos| {
        if grid.get_at_unbounded(pos) == '@' {
            let neighbors = grid
                .get_all_neighbors(pos)
                .iter().filter(|(pos, c)|{ *c == '@' }).count();

            if neighbors < 4 {
                accessible.push(pos);
            }
        }
    });

    accessible
}

pub fn part2(mut grid: Grid<char>) -> usize {
    let mut count = 0;

    loop {
        let removable = part1(&grid);
        if removable.is_empty() {
            break;
        }

        count += removable.len();

        for pos in removable {
            grid.set_at_unbounded(pos, '.');
        }
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
    use std::str::FromStr;
    use super::*;

    #[test]
    fn test_part1() {
        let mut grid = Grid::<char>::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(part1(&grid).len(), 13);
    }

    #[test]
    fn test_part2() {
        let mut grid = Grid::<char>::from_str(SAMPLE_INPUT).unwrap();
        assert_eq!(part2(grid), 43);
    }
}