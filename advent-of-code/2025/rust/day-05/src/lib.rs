use std::collections::HashSet;

use itertools::Itertools;

pub fn part1(input: &str) -> usize {
    // let mut fresh = HashSet::new();

    let (fresh_ranges, available) = input.split("\n\n").collect_tuple().unwrap();

    let fresh = fresh_ranges.lines().fold(HashSet::new(), |mut set, next| {
        let mut range = next.split('-');
        let start = range.next().unwrap().parse::<usize>().unwrap();
        let end = range.next().unwrap().parse::<usize>().unwrap();

        for i in start..=end {
            set.insert(i);
        };

        set
    });

    // dbg!(fresh);

    available.lines().filter(|line| fresh.contains(&line.parse::<usize>().unwrap())).count()
}

pub fn part2(_input: &str) -> String {

    todo!()
}

pub const SAMPLE_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT), 3);
    }
}