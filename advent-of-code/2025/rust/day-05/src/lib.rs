use itertools::Itertools;

/// Parses ranges, sorts them, and merges overlapping/adjacent intervals.
/// Returns a sorted Vec of non-overlapping (start, end) tuples.
fn parse_and_merge_ranges(ranges_str: &str) -> Vec<(usize, usize)> {
    let mut ranges: Vec<(usize, usize)> = ranges_str
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let start = parts.next().unwrap().parse::<usize>().unwrap();
            let end = parts.next().unwrap().parse::<usize>().unwrap();
            (start, end)
        })
        .collect();

    // Sort by start, then by end
    ranges.sort_unstable();

    // Merge overlapping/adjacent ranges
    let mut merged: Vec<(usize, usize)> = Vec::with_capacity(ranges.len());
    for (start, end) in ranges {
        if let Some(last) = merged.last_mut() {
            // If current range overlaps or is adjacent to the last merged range
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
                continue;
            }
        }
        merged.push((start, end));
    }

    merged
}

/// Binary search to check if a value is contained in any of the merged ranges.
fn contains_value(merged_ranges: &[(usize, usize)], value: usize) -> bool {
    // Binary search: find the rightmost range where start <= value
    let idx = merged_ranges.partition_point(|&(start, _)| start <= value);

    // If idx is 0, no range starts at or before value
    if idx == 0 {
        return false;
    }

    // Check if value falls within the range at idx-1
    let (start, end) = merged_ranges[idx - 1];
    value >= start && value <= end
}

pub fn part1(input: &str) -> usize {
    let (fresh_ranges, available) = input.split("\n\n").collect_tuple().unwrap();

    let merged = parse_and_merge_ranges(fresh_ranges);

    available
        .lines()
        .filter(|line| {
            let value = line.parse::<usize>().unwrap();
            contains_value(&merged, value)
        })
        .count()
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