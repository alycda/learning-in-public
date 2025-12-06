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
    let first_line = input.lines().next().unwrap();
    let capacity = first_line.split_whitespace().count();

    let mut matrix: Vec<Vec<String>> = vec![vec![]; capacity];

    let grid = Grid::<char>::from_str(input).unwrap();

    // Determine which logical column each character column belongs to
    // by finding word boundaries in the first row
    let mut col_mapping: Vec<Option<usize>> = vec![None; grid.get_width()];
    let mut logical_col = 0;
    let mut in_word = false;

    for char_col in 0..grid.get_width() {
        let c = grid.get_at_unbounded(Position::new(char_col as i32, 0));
        if c != ' ' {
            if !in_word {
                in_word = true;
            }
            col_mapping[char_col] = Some(logical_col);
        } else {
            if in_word {
                logical_col += 1;
                in_word = false;
            }
        }
    }

    // For each character column, read vertically and group by logical column
    for char_col in 0..grid.get_width() {
        if let Some(log_col) = col_mapping[char_col] {
            let mut vertical = String::new();
            for row in (0..grid.get_height()).step_by(size) {
                for offset in 0..size {
                    if row + offset < grid.get_height() {
                        let c = grid.get_at_unbounded(Position::new(char_col as i32, (row + offset) as i32));
                        vertical.push(c);
                    }
                }
            }
            matrix[log_col].push(vertical);
        }
    }

    // Apply operators to each logical column
    ops.split_whitespace().enumerate().map(|(col_idx, op)| {
        let vertical_numbers: Vec<u64> = matrix[col_idx]
            .iter()
            .map(|s| s.trim().parse::<u64>().unwrap_or(0))
            .filter(|&n| n > 0)
            .collect();

        if vertical_numbers.is_empty() {
            return 0;
        }

        vertical_numbers.iter().skip(1).fold(vertical_numbers[0], |acc, &next| {
            match op {
                "*" => acc * next,
                "+" => acc + next,
                _ => unreachable!()
            }
        })
    }).sum()
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