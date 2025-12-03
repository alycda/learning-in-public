fn part1(input: &str) -> u32 {
    input.lines()
        .fold(vec![0], |mut acc, line| {
            // dbg!(line.chars());

            acc.push(line.chars().collect::<Vec<_>>().windows(2).map(|w| {
                // dbg!(w);

                w.iter().fold(0, |acc, c| acc * 10 + c.to_digit(10).unwrap_or(0))
            }).max().unwrap());

            dbg!(acc)
        }).iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("987654321111111
811111111111119
234234234234278
818181911112111"), 357);
    }
}