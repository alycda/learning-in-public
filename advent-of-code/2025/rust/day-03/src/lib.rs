//! Day 03: Joltage Selection
//!
//! Find the N largest digits in order from each line.
//! Part 1: 2 digits, Part 2: 12 digits
//!
//! Implementations:
//! - Default: Iterator-based with greedy selection
//! - `regex` feature: Uses Rust regex crate for digit extraction
//! - `pcre2` feature: Uses pcre2 crate (wraps libpcre2-8) for benchmarking

/// Sample input for CI (real input.txt is gitignored)
pub const SAMPLE_INPUT: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

// ============================================================================
// Iterator-based implementation (default)
// ============================================================================

/// Greedy selection of N largest digits in order using iterators
fn select_n_largest_iter(line: &str, n: usize) -> Vec<u32> {
    let char_count = line.len();
    if char_count < n {
        return vec![];
    }

    let digits: Vec<u32> = line.bytes().map(|b| (b - b'0') as u32).collect();
    let mut result = Vec::with_capacity(n);
    let mut pos = 0;

    for i in 0..n {
        let remaining = n - i - 1;
        let end = char_count - remaining;

        // Find max digit in range [pos, end), preferring rightmost on tie
        let (best_pos, best_val) = digits[pos..end]
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.cmp(b.1).then(std::cmp::Ordering::Greater))
            .map(|(idx, &val)| (pos + idx, val))
            .unwrap();

        result.push(best_val);
        pos = best_pos + 1;
    }

    result
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let digits = select_n_largest_iter(line, 2);
            digits[0] * 10 + digits[1]
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let digits = select_n_largest_iter(line, 12);
            digits
                .iter()
                .fold(0u64, |acc, &d| acc * 10 + d as u64)
        })
        .sum()
}

// ============================================================================
// Regex-based implementation
// ============================================================================

#[cfg(feature = "regex")]
pub mod regex_impl {
    use regex::Regex;
    use std::sync::LazyLock;

    static DIGIT_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d").unwrap());

    /// Extract all digits with their positions using regex
    fn extract_digits(line: &str) -> Vec<(usize, u32)> {
        DIGIT_RE
            .find_iter(line)
            .map(|m| {
                let digit = m.as_str().parse::<u32>().unwrap();
                (m.start(), digit)
            })
            .collect()
    }

    /// Greedy selection of N largest digits in order using regex-extracted positions
    fn select_n_largest_regex(line: &str, n: usize) -> Vec<u32> {
        let digits = extract_digits(line);
        let len = digits.len();
        if len < n {
            return vec![];
        }

        let mut result = Vec::with_capacity(n);
        let mut idx = 0;

        for i in 0..n {
            let remaining = n - i - 1;
            let end_idx = len - remaining;

            // Find max digit in range [idx, end_idx), preferring rightmost on tie
            let (best_idx, &(_, best_val)) = digits[idx..end_idx]
                .iter()
                .enumerate()
                .max_by(|a, b| a.1 .1.cmp(&b.1 .1).then(std::cmp::Ordering::Greater))
                .map(|(i, d)| (idx + i, d))
                .unwrap();

            result.push(best_val);
            idx = best_idx + 1;
        }

        result
    }

    pub fn part1(input: &str) -> u32 {
        input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|line| {
                let digits = select_n_largest_regex(line, 2);
                digits[0] * 10 + digits[1]
            })
            .sum()
    }

    pub fn part2(input: &str) -> u64 {
        input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|line| {
                let digits = select_n_largest_regex(line, 12);
                digits
                    .iter()
                    .fold(0u64, |acc, &d| acc * 10 + d as u64)
            })
            .sum()
    }
}

// ============================================================================
// PCRE2 implementation (using pcre2 crate which wraps libpcre2)
// ============================================================================

#[cfg(feature = "pcre2")]
pub mod pcre2_impl {
    use pcre2::bytes::Regex;
    use std::sync::LazyLock;

    static DIGIT_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d").unwrap());

    /// Extract all digits with their positions using PCRE2
    fn extract_digits(line: &str) -> Vec<(usize, u32)> {
        DIGIT_RE
            .find_iter(line.as_bytes())
            .filter_map(|m| m.ok())
            .map(|m| {
                let digit = (line.as_bytes()[m.start()] - b'0') as u32;
                (m.start(), digit)
            })
            .collect()
    }

    /// Greedy selection of N largest digits in order using PCRE2-extracted positions
    fn select_n_largest_pcre2(line: &str, n: usize) -> Vec<u32> {
        let digits = extract_digits(line);
        let len = digits.len();
        if len < n {
            return vec![];
        }

        let mut result = Vec::with_capacity(n);
        let mut idx = 0;

        for i in 0..n {
            let remaining = n - i - 1;
            let end_idx = len - remaining;

            // Find max digit in range [idx, end_idx), preferring rightmost on tie
            let (best_idx, &(_, best_val)) = digits[idx..end_idx]
                .iter()
                .enumerate()
                .max_by(|a, b| a.1 .1.cmp(&b.1 .1).then(std::cmp::Ordering::Greater))
                .map(|(i, d)| (idx + i, d))
                .unwrap();

            result.push(best_val);
            idx = best_idx + 1;
        }

        result
    }

    pub fn part1(input: &str) -> u32 {
        input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|line| {
                let digits = select_n_largest_pcre2(line, 2);
                digits[0] * 10 + digits[1]
            })
            .sum()
    }

    pub fn part2(input: &str) -> u64 {
        input
            .lines()
            .filter(|l| !l.is_empty())
            .map(|line| {
                let digits = select_n_largest_pcre2(line, 12);
                digits
                    .iter()
                    .fold(0u64, |acc, &d| acc * 10 + d as u64)
            })
            .sum()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "987654321111111
811111111111119
234234234234278
818181911112111"
            ),
            357
        );
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
    fn joltage_2(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(part1(input), expected);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "987654321111111
811111111111119
234234234234278
818181911112111"
            ),
            3121910778619
        );
    }

    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    fn joltage_12(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(part2(input), expected);
    }

    // Regex implementation tests (when feature enabled)
    #[cfg(feature = "regex")]
    mod regex_tests {
        use super::super::regex_impl;
        use rstest::rstest;

        #[test]
        fn test_regex_part1() {
            assert_eq!(
                regex_impl::part1(
                    "987654321111111
811111111111119
234234234234278
818181911112111"
                ),
                357
            );
        }

        #[rstest]
        #[case("987654321111111", 98)]
        #[case("811111111111119", 89)]
        #[case("234234234234278", 78)]
        #[case("818181911112111", 92)]
        fn regex_joltage_2(#[case] input: &str, #[case] expected: u32) {
            assert_eq!(regex_impl::part1(input), expected);
        }

        #[test]
        fn test_regex_part2() {
            assert_eq!(
                regex_impl::part2(
                    "987654321111111
811111111111119
234234234234278
818181911112111"
                ),
                3121910778619
            );
        }

        #[rstest]
        #[case("987654321111111", 987654321111)]
        #[case("811111111111119", 811111111119)]
        #[case("234234234234278", 434234234278)]
        #[case("818181911112111", 888911112111)]
        fn regex_joltage_12(#[case] input: &str, #[case] expected: u64) {
            assert_eq!(regex_impl::part2(input), expected);
        }

        // Cross-check: both implementations should produce same results
        #[rstest]
        #[case("987654321111111")]
        #[case("811111111111119")]
        #[case("234234234234278")]
        #[case("818181911112111")]
        fn implementations_match(#[case] input: &str) {
            assert_eq!(super::part1(input), regex_impl::part1(input));
            assert_eq!(super::part2(input), regex_impl::part2(input));
        }
    }

    // PCRE2 implementation tests (when feature enabled)
    #[cfg(feature = "pcre2")]
    mod pcre2_tests {
        use super::super::pcre2_impl;
        use rstest::rstest;

        #[test]
        fn test_pcre2_part1() {
            assert_eq!(
                pcre2_impl::part1(
                    "987654321111111
811111111111119
234234234234278
818181911112111"
                ),
                357
            );
        }

        #[rstest]
        #[case("987654321111111", 98)]
        #[case("811111111111119", 89)]
        #[case("234234234234278", 78)]
        #[case("818181911112111", 92)]
        fn pcre2_joltage_2(#[case] input: &str, #[case] expected: u32) {
            assert_eq!(pcre2_impl::part1(input), expected);
        }

        #[test]
        fn test_pcre2_part2() {
            assert_eq!(
                pcre2_impl::part2(
                    "987654321111111
811111111111119
234234234234278
818181911112111"
                ),
                3121910778619
            );
        }

        #[rstest]
        #[case("987654321111111", 987654321111)]
        #[case("811111111111119", 811111111119)]
        #[case("234234234234278", 434234234278)]
        #[case("818181911112111", 888911112111)]
        fn pcre2_joltage_12(#[case] input: &str, #[case] expected: u64) {
            assert_eq!(pcre2_impl::part2(input), expected);
        }

        // Cross-check: pcre2 should match default implementation
        #[rstest]
        #[case("987654321111111")]
        #[case("811111111111119")]
        #[case("234234234234278")]
        #[case("818181911112111")]
        fn pcre2_matches_default(#[case] input: &str) {
            assert_eq!(super::part1(input), pcre2_impl::part1(input));
            assert_eq!(super::part2(input), pcre2_impl::part2(input));
        }
    }
}
