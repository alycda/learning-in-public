
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
    let capacity = first.split_whitespace().count();

    // Collect numbers for each column (same as part1)
    let mut columns: Vec<Vec<String>> = vec![vec![]; capacity];

    for line in input.lines() {
        for (idx, num) in line.split_whitespace().enumerate() {
            columns[idx].push(num.to_string());
        }
    }

    // For each column, align based on operator and read vertically
    ops.split_whitespace().enumerate().map(|(col_idx, op)| {
        let col = &columns[col_idx];
        let max_width = col.iter().map(|n| n.len()).max().unwrap_or(0);

        // Operator determines alignment: * = right-align, + = left-align
        let padded: Vec<String> = col.iter()
            .map(|n| {
                if op == "*" {
                    format!("{:>width$}", n, width = max_width) // right-align
                } else {
                    format!("{:<width$}", n, width = max_width) // left-align
                }
            })
            .collect();

        // Read positions: * reads right-to-left, + reads left-to-right
        let positions: Vec<usize> = if op == "*" {
            (0..max_width).rev().collect()
        } else {
            (0..max_width).collect()
        };

        let vertical_numbers: Vec<u64> = positions.iter()
            .map(|&pos| {
                let vertical: String = padded.iter()
                    .map(|s| s.chars().nth(pos).unwrap())
                    .collect();
                vertical.trim().parse::<u64>().unwrap_or(0)
            })
            .filter(|&n| n > 0)
            .collect();

        // Apply operator
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