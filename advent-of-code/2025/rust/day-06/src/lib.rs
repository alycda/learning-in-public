use std::str::FromStr;

use aoc_ornaments::spatial::{Grid, Position};

pub fn part1(input: &str, ops: &str) -> u64 {
    let mut lines = input.lines().peekable();

    let first = lines.peek().unwrap();
    let capacity = dbg!(first.split_whitespace().count());

    let mut matrix = vec![vec![]; capacity];

    lines.for_each(|line| {
        line.split_whitespace().enumerate().for_each(|(idx, num)| {
            // dbg!(num.parse::<u64>().unwrap());

            matrix[idx].push(num.parse::<u64>().unwrap());
        });
    });

    // dbg!(&matrix);

    ops.split_whitespace().enumerate().map(|(idx, op)| {
        dbg!(idx, op);

        matrix[idx].iter().skip(1).fold(matrix[idx][0], |acc, next| {
            match op {
                "*" => { acc * next }
                "+" => { acc + next }
                _ => unreachable!()
            }
        })
    }).sum()
}

/// assume max 4 digits
pub fn part2(input: &str, ops: &str, size: usize) -> u64 {
    let mut lines = input.lines().peekable();

    let first = lines.peek().unwrap();
    let capacity = dbg!(first.split_whitespace().count());

    let mut matrix = vec![vec![]; capacity];

    let grid = Grid::<char>::from_str(input).unwrap();

    // dbg!(&grid);

    for col in 0..grid.get_width() {
        // dbg!(col);

        for row in (0..grid.get_height()).step_by(size) {
            // dbg!(row);

            let a = Position::new(col as i32, row as i32);
            let b = Position::new(col as i32, (row + 1) as i32);
            let c = Position::new(col as i32, (row + 2) as i32);
            let d = Position::new(col as i32, (row + 2) as i32);

            let a = dbg!(grid.get_at_unbounded(a));
            let b = dbg!(grid.get_at_unbounded(b));
            let c = dbg!(grid.get_at_unbounded(c));

            if size == 4 {
                dbg!(grid.get_at_unbounded(d));
            }

            let s: String = vec![a,b,c].iter().collect();

            matrix[row].push( s );
        }
    }

    dbg!(&matrix);

    matrix[0].iter()
        // .filter(|v| !v.is_empty())
        .map(|s| s.trim().parse::<u64>().unwrap_or(0))
        .for_each(|num|{
            dbg!(num);
        });

    todo!()
}

pub const SAMPLE_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314";

pub const OPS: &str = "*   +   *   +  ";

#[cfg(test)]
mod tests {
    use super::*;
    // use rstest::rstest;

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT, OPS), 4277556);
    }

    // #[rstest]
    // #[case("64 23 314", "+", 1058)] // 4 + 431 + 623
    // #[case("51 387 215", "*", 3253600)] // 175 * 581 * 32
    // #[case("328 64 98", "+", 625)] // 8 + 248 + 369
    // #[case("123 45 6", "+", 8544)] // 356 * 24 * 1
    // fn test_cases(#[case] input: &str, #[case] op: &str, #[case] expected: u64) {
    //     assert_eq!(part2(input, op), expected);
    // }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE_INPUT, OPS, 3), 3263827);
    }
}