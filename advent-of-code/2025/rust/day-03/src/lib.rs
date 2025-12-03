pub fn part1(input: &str) -> u32 {
    input.lines()
        .fold(vec![0], |mut acc, line| {
            let chars = line.chars();
            let char_count = line.chars().count();

            // dbg!(char_count);

            let mut nums = chars.map(|c| c.to_digit(10));
            let nums_clone = nums.clone();
            // dbg!(&nums);

            // .rev() because if there are multiple maxes, the last one is returned. can't do this, breaks case #2
            let max = nums.take(char_count-1)
                .enumerate()
                .max_by(|a, b| {
                    a.1.cmp(&b.1).then(std::cmp::Ordering::Greater)
                })
                // .max_by_key(|(_, val)| *val)
                // .map(|(idx, _)| idx)
                .unwrap();

            // let first_max = nums.find(|e |e.unwrap() == max.1.unwrap());

            let max_idx = max.0;
            dbg!(max_idx);


            let next_max = nums_clone.skip(max_idx+1)
                // .enumerate()
                // .max_by_key(|(_, val)| *val)
                // .map(|(idx, _)| idx)
                .max()
                .unwrap();
            // dbg!(index_of_next_max + max_idx + 1);

            dbg!(max.1, next_max);

            acc.push(max.1.unwrap() * 10 + next_max.unwrap());

            // acc.push(nums.nth(index_of_max.unwrap()).unwrap() * 10 + nums.nth(index_of_next_max.unwrap()).unwrap());

            // if index_of_max.unwrap() > line.chars().count() {
            //     // return line.chars().skip(line.chars().count()-2).fold(0, |acc, c| acc * 10 + c.to_digit(10).unwrap_or(0))
            // } 

            // todo!("from rev find the highest number index, then compare to all following indices to find the next highest number");

            // acc.push(line.chars().collect::<Vec<_>>().windows(2).map(|w| {
            //     // dbg!(w);

            //     w.iter().fold(0, |acc, c| acc * 10 + c.to_digit(10).unwrap_or(0))
            // }).max().unwrap());

            acc
        }).iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

//     #[test]
//     fn test_part1() {
//         assert_eq!(part1("987654321111111
// 811111111111119
// 234234234234278
// 818181911112111"), 357);
//     }

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    #[case("2232133233333122223222321121432322323324333234233221423334362333113343833132233313523312322224432234", 85)]
    #[case("9544718948279477416294977734546287758964484675984344448638555429875995525496945633428322464129775449", 99)]
    #[case("2342431222323242222322132121222212212132223222221322252221222222422212122222231227223222421241422222", 74)]
    #[case("6244953925232293122334482643333513353336433435373235433333433333344373324258246634153623454355543453", 99)]
    // #[case("", 0)]
    // #[case("", 0)]
    // #[case("", 0)]
    // #[case("", 0)]
    // #[case("", 0)]
    // #[case("", 0)]
    // #[case("", 0)]
    // #[case("", 0)]
    // #[case("", 0)]
    // #[case("", 0)]
    fn joltage(#[case] input: &str,#[case] expected: u32) {
        assert_eq!(part1(input), expected);
    }
}