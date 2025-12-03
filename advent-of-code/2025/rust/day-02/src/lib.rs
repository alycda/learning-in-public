//! Day 02: Repeating Patterns
//!
//! Find numbers in ranges that have repeating digit patterns.
//! Part 1: Numbers with repeating halves (e.g., 1212, 123123)
//! Part 2: Numbers with any repeating pattern
//!
//! Implementations:
//! - Default: Iterator-based string manipulation
//! - `regex` feature: Uses regex for parsing (no backrefs for patterns)
//! - `pcre2` feature: Uses PCRE2 backreferences for pattern matching

// ============================================================================
// Iterator-based implementation (default)
// ============================================================================

fn parse_ranges(input: &str) -> impl Iterator<Item = (u128, u128)> + '_ {
    input
        .lines()
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

fn largest_repeating_pattern(s: &str) -> Option<&str> {
    let n = s.len();

    (1..=n / 2)
        .rev()
        .filter(|&len| n.is_multiple_of(len))
        .find(|&len| s[..len].repeat(n / len) == s)
        .map(|len| &s[..len])
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

// ============================================================================
// Regex-based implementation
// Note: Rust regex doesn't support backreferences, so pattern matching
// still uses string manipulation. Regex is used for parsing.
// ============================================================================

#[cfg(feature = "regex")]
pub mod regex_impl {
    use regex::Regex;
    use std::sync::LazyLock;

    static RANGE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+)-(\d+)").unwrap());

    fn parse_ranges(input: &str) -> Vec<(u128, u128)> {
        RANGE_RE
            .captures_iter(input)
            .map(|cap| {
                let a: u128 = cap[1].parse().unwrap();
                let b: u128 = cap[2].parse().unwrap();
                (a, b)
            })
            .collect()
    }

    fn has_repeating_halves(n: u64) -> bool {
        let s = n.to_string();
        let count = s.len();

        if count % 2 != 0 {
            return false;
        }

        let mid = count / 2;
        s[..mid] == s[mid..]
    }

    fn largest_repeating_pattern(s: &str) -> Option<usize> {
        let n = s.len();

        (1..=n / 2)
            .rev()
            .filter(|&len| n % len == 0)
            .find(|&len| s[..len].repeat(n / len) == s)
    }

    pub fn part1(input: &str) -> String {
        parse_ranges(input)
            .into_iter()
            .flat_map(|(a, b)| (a as u64)..=(b as u64))
            .filter(|&i| has_repeating_halves(i))
            .sum::<u64>()
            .to_string()
    }

    pub fn part2(input: &str) -> String {
        parse_ranges(input)
            .into_iter()
            .flat_map(|(a, b)| a..=b)
            .filter(|&i| largest_repeating_pattern(&i.to_string()).is_some())
            .sum::<u128>()
            .to_string()
    }
}

// ============================================================================
// PCRE2 implementation
// PCRE2 supports backreferences! We can use ^(.+)\1+$ to match repeating patterns
// ============================================================================

#[cfg(feature = "pcre2")]
pub mod pcre2_impl {
    use pcre2::bytes::Regex;
    use std::sync::LazyLock;

    static RANGE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+)-(\d+)").unwrap());

    // Backreference pattern: matches strings made entirely of a repeated substring
    // ^(.+)\1+$ means: capture something, then match that same thing one or more times
    static REPEATING_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(.+)\1+$").unwrap());

    fn parse_ranges(input: &str) -> Vec<(u128, u128)> {
        RANGE_RE
            .captures_iter(input.as_bytes())
            .filter_map(|cap| cap.ok())
            .map(|cap| {
                let a: u128 = std::str::from_utf8(&cap[1]).unwrap().parse().unwrap();
                let b: u128 = std::str::from_utf8(&cap[2]).unwrap().parse().unwrap();
                (a, b)
            })
            .collect()
    }

    fn has_repeating_halves(n: u64) -> bool {
        let s = n.to_string();
        let count = s.len();

        if count % 2 != 0 {
            return false;
        }

        let mid = count / 2;
        s[..mid] == s[mid..]
    }

    /// Check if string has a repeating pattern using PCRE2 backreferences
    fn has_repeating_pattern(s: &str) -> bool {
        REPEATING_RE.is_match(s.as_bytes()).unwrap_or(false)
    }

    pub fn part1(input: &str) -> String {
        parse_ranges(input)
            .into_iter()
            .flat_map(|(a, b)| (a as u64)..=(b as u64))
            .filter(|&i| has_repeating_halves(i))
            .sum::<u64>()
            .to_string()
    }

    pub fn part2(input: &str) -> String {
        parse_ranges(input)
            .into_iter()
            .flat_map(|(a, b)| a..=b)
            .filter(|&i| has_repeating_pattern(&i.to_string()))
            .sum::<u128>()
            .to_string()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "1227775554".to_string());
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
    #[case("446446", Some("446"))]
    #[case("38593859", Some("3859"))]
    #[case("565656", Some("56"))]
    #[case("824824824", Some("824"))]
    #[case("2121212121", Some("21"))]
    fn repeating_patterns(#[case] input: &str, #[case] expected: Option<&str>) {
        assert_eq!(largest_repeating_pattern(input), expected);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "4174379265".to_string());
    }

    // Regex implementation tests
    #[cfg(feature = "regex")]
    mod regex_tests {
        use super::super::regex_impl;
        use super::TEST_INPUT;

        #[test]
        fn test_regex_part1() {
            assert_eq!(regex_impl::part1(TEST_INPUT), "1227775554".to_string());
        }

        #[test]
        fn test_regex_part2() {
            assert_eq!(regex_impl::part2(TEST_INPUT), "4174379265".to_string());
        }

        #[test]
        fn implementations_match() {
            assert_eq!(super::part1(TEST_INPUT), regex_impl::part1(TEST_INPUT));
            assert_eq!(super::part2(TEST_INPUT), regex_impl::part2(TEST_INPUT));
        }
    }

    // PCRE2 implementation tests
    #[cfg(feature = "pcre2")]
    mod pcre2_tests {
        use super::super::pcre2_impl;
        use super::TEST_INPUT;

        #[test]
        fn test_pcre2_part1() {
            assert_eq!(pcre2_impl::part1(TEST_INPUT), "1227775554".to_string());
        }

        #[test]
        fn test_pcre2_part2() {
            assert_eq!(pcre2_impl::part2(TEST_INPUT), "4174379265".to_string());
        }

        #[test]
        fn implementations_match() {
            assert_eq!(super::part1(TEST_INPUT), pcre2_impl::part1(TEST_INPUT));
            assert_eq!(super::part2(TEST_INPUT), pcre2_impl::part2(TEST_INPUT));
        }
    }
}
