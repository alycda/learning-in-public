fn parse_ranges(input: &str) -> impl Iterator<Item = (u128, u128)> + '_ {
    input.lines()
        .flat_map(|line| line.split(','))
        .map(|id| {
            let (a, b) = id.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
}

fn has_repeating_halves(n: u64) -> bool {
    let s = n.to_string();
    let count = s.len();

    if !count.is_multiple_of(2) {
        return false;
    }

    let mid = count / 2;
    let mut chars = s.chars();
    chars.by_ref().take(mid).collect::<Vec<_>>() == chars.take(mid).collect::<Vec<_>>()
}

pub fn part1(input: &str) -> String {
    parse_ranges(input)
        .flat_map(|(a, b)| (a as u64)..=(b as u64))
        .filter(|&i| has_repeating_halves(i))
        .sum::<u64>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    parse_ranges(input)
        .flat_map(|(a, b)| a..=b)
        .filter(|&i| largest_repeating_pattern(&i.to_string()).is_some())
        .sum::<u128>()
        .to_string()
}

fn largest_repeating_pattern(s: &str) -> Option<&str> {
    let n = s.len();

    (1..=n / 2)
        .rev()
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