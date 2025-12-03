fn part1(input: &str) -> u32 {
    input.lines()
        .fold(vec![0], |mut acc, line| {
            let chars = line.chars();
            let char_count = line.chars().count();

            dbg!(char_count);

            let nums = chars.map(|c| c.to_digit(10));
            let nums_clone = nums.clone();
            // dbg!(&nums);

            let max = nums.take(char_count-1)
                .enumerate()
                .max_by_key(|(_, val)| *val)
                // .map(|(idx, _)| idx)
                .unwrap();

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

    #[test]
    fn test_part1() {
        assert_eq!(part1("987654321111111
811111111111119
234234234234278
818181911112111"), 357);
    }
}