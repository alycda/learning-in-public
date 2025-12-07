use std::{collections::{HashMap, HashSet}, str::FromStr};

use aoc_ornaments::spatial::{Grid, Position};

pub fn part1(input: &str) -> usize {
    let start = input.lines().next().unwrap().chars().enumerate().find(|(_idx, c)| *c == 'S').unwrap();

    let mut splits_hit = HashSet::new();
    let mut active_cols = HashSet::from([start.0]);
    let grid = Grid::<char>::from_str(input).unwrap();

    for row in 1..grid.get_height() {
        let mut next_cols = HashSet::new();

        for col in &active_cols {
            let c = grid.get_at_unbounded(Position::new(*col as i32, row as i32));

            match c {
                '^' => {
                    splits_hit.insert((row, *col));
                    next_cols.insert(col - 1);
                    next_cols.insert(col + 1);
                }
                _ => {
                    next_cols.insert(*col);
                }
            }
        }

        active_cols = next_cols;
    };

    splits_hit.len()
}

fn count_paths(grid: &Grid<char>, row: usize, col: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
    if row >= grid.get_height() {
        return 1;
    }

    if let Some(&cached) = memo.get(&(row, col)) {
        return cached;
    }

    let result = match grid.get_at_unbounded(Position::new(col as i32, row as i32)) {
        '^' => {
            let left = if col > 0 {
                count_paths(grid, row + 1, col - 1, memo)
            } else {
                0
            };
            let right = if col + 1 < grid.get_width() {
                count_paths(grid, row + 1, col + 1, memo)
            } else {
                0
            };
            left + right
        }
        _ => count_paths(grid, row + 1, col, memo)
    };

    memo.insert((row, col), result);
    result
}

pub fn part2(input: &str) -> usize {
    let mut memo = HashMap::new();
    let grid = Grid::<char>::from_str(input).unwrap();
    let start = input.lines().next().unwrap().chars().enumerate().find(|(_idx, c)| *c == 'S').unwrap();

    count_paths(&grid, 0, start.0, &mut memo)
}

pub const SAMPLE_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_INPUT), 40);
    }
}