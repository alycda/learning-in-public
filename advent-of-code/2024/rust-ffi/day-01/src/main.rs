//! 2024 Day 1: Historian Hysteria

/// transpose 2 colums of numbers
fn unzip(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| {
            let nums = line
                // from columns
                .split_whitespace()
                // parse number
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            (nums[0], nums[1])
        })
        .unzip()
}

fn process(input: &str) -> Result<i32, String> { 
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = unzip(input);

    // critical: sort both lists
    left.sort();
    right.sort();

    Ok(left
        .iter()
        // for each element
        .zip(right.iter())
        // get the absolute difference
        .map(|(l, r)| (l-r).abs())
        // and sum
        .sum::<i32>()
    )
}

fn main() {
    println!("2024 Day 1");

    assert_eq!(process("3 7").unwrap(), 4);
    assert_eq!(process("9 3").unwrap(), 6);

    println!("Part 1: {}", process("3   4
4   3
2   5
1   3
3   9
3   3").unwrap())
}
