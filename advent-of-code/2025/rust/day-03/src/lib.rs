pub fn part1(input: &str) -> u32 {
    input.lines()
        .fold(vec![0], |mut acc, line| {
            let chars = line.chars();
            let char_count = line.chars().count();

            let nums = chars.map(|c| c.to_digit(10));
            let nums_clone = nums.clone();

            let max = nums.take(char_count-1)
                .enumerate()
                .max_by(|a, b| {
                    a.1.cmp(&b.1).then(std::cmp::Ordering::Greater)
                })
                .unwrap();

            let max_idx = max.0;
            dbg!(max_idx);

            let next_max = nums_clone.skip(max_idx+1)
                .max()
                .unwrap();

            dbg!(max.1, next_max);

            acc.push(max.1.unwrap() * 10 + next_max.unwrap());

            acc
        }).iter().sum()
}

pub fn part2(input: &str) -> u64 {
    input.lines()
        .fold(vec![0], |mut acc, line| {
            let chars = line.chars();
            let char_count = line.chars().count();

            let nums = chars.map(|c| c.to_digit(10));
            // let _nums_clone = nums.clone();

            let max = nums.take(char_count-12)
                .enumerate()
                .max_by(|a, b| {
                    a.1.cmp(&b.1).then(std::cmp::Ordering::Greater)
                })
                .unwrap();

            let mut max_idx = max.0;
            dbg!(max_idx);
            let mut joltage = vec![max.1.unwrap()];

            for _ in 0..11 {
                let nums_clone = line.chars().map(|c| c.to_digit(10));
                let next_max = nums_clone.skip(max_idx+1)
                    .enumerate()
                    .max_by(|a, b| {
                        a.1.cmp(&b.1).then(std::cmp::Ordering::Greater)
                    })
                    .unwrap();

                max_idx = next_max.0 + max_idx + 1;
                joltage.push(next_max.1.unwrap());
            }

            // dbg!(&joltage);

            acc.push(joltage.iter().map(|d| char::from_digit(*d, 10).unwrap()).collect::<String>().parse::<u64>().unwrap());

            acc
        }).iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part1() {
        assert_eq!(part1("987654321111111
811111111111119
234234234234278
818181911112111"), 357);
    }

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    #[case("2232133233333122223222321121432322323324333234233221423334362333113343833132233313523312322224432234", 85)]
    #[case("9544718948279477416294977734546287758964484675984344448638555429875995525496945633428322464129775449", 99)]
    #[case("2342431222323242222322132121222212212132223222221322252221222222422212122222231227223222421241422222", 74)]
    #[case("6244953925232293122334482643333513353336433435373235433333433333344373324258246634153623454355543453", 99)]
    fn joltage_2(#[case] input: &str,#[case] expected: u32) {
        assert_eq!(part1(input), expected);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("987654321111111
811111111111119
234234234234278
818181911112111"), 3121910778619);
    }

    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    // #[case("", 0)]
    // #[case("", 0)]
    // #[case("", 0)]
    // #[case("", 0)]
    // #[case("", 0)]
    // #[case("", 0)]
    fn joltage_12(#[case] input: &str,#[case] expected: u64) {
        assert_eq!(part2(input), expected);
    }

}