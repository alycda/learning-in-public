pub fn part1(input: &str) -> String {
    let mut invalid_ids = vec![0];

    input.lines().for_each(|line| {
        line.split(',')
            .for_each(|id| {
                // dbg!(&id);
                let mut ids = id.split('-');
                let a = ids.next().unwrap().parse::<u64>().unwrap();
                let b = ids.next().unwrap().parse::<u64>().unwrap();

                // dbg!(a, b);

                for i in a..=b {
                    let s = i.to_string();
                    let count = s.chars().count();

                    if count.is_multiple_of(2) {
                        // dbg!(i);
                        let mid = count / 2;
                        let mut chars = s.chars();

                        if chars.by_ref().take(mid).collect::<Vec<_>>() == chars.by_ref().take(mid).collect::<Vec<_>>() {
                            // dbg!(i);
                            invalid_ids.push(i);
                        }
                    }
                }
            });
    });

    invalid_ids.iter().sum::<u64>().to_string()
}

pub fn part2(input: &str) -> String {
    let mut invalid_ids = vec![0_u128];

    input.lines().for_each(|line| {
        line.split(',')
            .for_each(|id| {
                // dbg!(&id);
                let mut ids = id.split('-');
                let a = ids.next().unwrap().parse::<u128>().unwrap();
                let b = ids.next().unwrap().parse::<u128>().unwrap();

                // dbg!(a, b);

                for i in a..=b {
                    let s = i.to_string();

                    // need whole number divisor
                    if let Some(_pattern) = largest_repeating_pattern(&s) {
                        invalid_ids.push(i);
                    }
                }
            });
    });

    invalid_ids.iter().sum::<u128>().to_string()
}

fn largest_repeating_pattern(s: &str) -> Option<&str> {
    let n = s.len();

    (1..=n / 2)
        .rev()
        // .inspect(|divisor| { dbg!(divisor); })
        .filter(|&len| n.is_multiple_of(len))
        .find(|&len| s[..len].repeat(n / len) == s)
        .map(|len| &s[..len])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part1() {
        assert_eq!(part1("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"), "1227775554".to_string());
    }

    #[rstest]
    #[case("11", Some("1"))]
    #[case("22", Some("2"))]
    #[case("99", Some("9"))]
    #[case("111", Some("1"))]
    #[case("999", Some("9"))]
    #[case("1010", Some("10"))]
    #[case("1188511885", Some("11885"))]
    #[case("222222", Some("222"))]
    // #[case("222222", Some("22"))]
    #[case("446446", Some("446"))]
    #[case("38593859", Some("3859"))]
    #[case("565656", Some("56"))]
    #[case("824824824", Some("824"))]
    #[case("2121212121", Some("21"))]
    fn repeating_patterns(#[case] input: &str,#[case] expected: Option<&str>) {
        assert_eq!(largest_repeating_pattern(input), expected);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"), "4174379265".to_string());
    }
}