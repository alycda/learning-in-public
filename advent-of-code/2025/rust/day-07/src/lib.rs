use std::{collections::HashSet, str::FromStr};

use aoc_ornaments::spatial::{Grid, Position};

pub fn part1(input: &str) -> usize {
    let start = input.lines().next().unwrap().chars().enumerate().find(|(_idx, c)| *c == 'S').unwrap();
    // let mut beams = vec![(0, start.0)];

    // let mut beams = HashSet::<usize>::new();
    // beams.insert(start.0);

    // let mut splits = HashSet::new();

    // // dbg!(start.0, &beams);

    let mut splits_hit = HashSet::new();
    let mut active_cols = HashSet::from([start.0]);
    let grid = Grid::<char>::from_str(input).unwrap();

    // grid.walk(|pos| {

    // });

    // // dbg!(&grid);

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

    //     dbg!(c);

    //     match c {
    //         '^' => {
    //             // remove the beam from the previous row and replace it with 2 new beams
    //             beams.remove(&7);
    //             splits.insert(7);
    //             beams.insert(6);
    //             beams.insert(8);
    //         }
    //         _ => {}
    //     }
    };



    // // input.lines().map(|line| {
    // //     line.chars().fold(0, |mut acc, c| {
    // //         if c == '^' {
    // //             acc +=1;
    // //         }

    // //         acc
    // //     })
    // // }).sum::<usize>() - 1

    splits_hit.len()
}

pub fn part2(input: &str) -> usize {
    todo!()
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
//     use rstest::rstest;

//     #[rstest]
//     #[case(".......S.......", 0)]
//     #[case(".......S.......
// ...............", 0)]
//     #[case(".......S.......
// ...............
// .......^.......", 1)]
//     #[case(".......S.......
// .......|.......
// ......|^|......
// ......|.|......
// ......^.^......", 3)]
//     // #[case("...", 0)]
//     // #[case("...", 0)]
//     // #[case("...", 0)]
//     // #[case("...", 0)]
//     // #[case("...", 0)]
//     // #[case("...", 0)]
//     // #[case("...", 0)]
//     fn test_cases(#[case] input: &str, #[case] expected: usize) {
//         assert_eq!(part1(input), expected);
//     }

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT), 21);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(SAMPLE_INPUT), 0);
    // }
}