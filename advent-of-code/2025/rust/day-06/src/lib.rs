pub fn part1(input: &str, ops: &str) -> u64 {
    let mut lines = input.lines().peekable();

    let first = lines.peek().unwrap();
    let capacity = dbg!(first.split_whitespace().count());

    let mut matrix = vec![vec![]; capacity];

    lines.for_each(|line| {
        let mut items = line.split_whitespace();

        items.enumerate().for_each(|(idx, num)| {
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

pub fn part2(_input: &str) -> String {
    todo!()
}

pub const SAMPLE_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314";

pub const OPS: &str = "*   +   *   +  ";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT, OPS), 4277556);
    }
}