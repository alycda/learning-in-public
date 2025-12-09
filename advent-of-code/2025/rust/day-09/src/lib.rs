use glam::IVec2;
use itertools::Itertools;

pub fn part1(input: &str) -> i64 {
    let points: Vec<IVec2> = input.lines().fold(vec![], |mut acc, line| {
        acc.push(IVec2::from_slice(&line.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>()));

        acc
    });

    // dbg!(&points);

    let max = points
        .iter()
        .combinations(2)
        .filter_map(|pair| {
            let a = pair[0];
            let b = pair[1];

            let c = dbg!((a - b).abs());

            // Area of the rectangle spanned by a and b
            let area:i64 = (c.x as i64 + 1) * (c.y as i64 + 1);

            // Return the two opposite corners and the area
            Some((*a, *b, area))
        })
        // .inspect(|pair|{ dbg!(pair); })
        .max_by_key(|(_, _, area)| *area);
    
    dbg!(max).unwrap().2
}

pub fn part2(input: &str) -> usize {
    todo!()
}

pub const SAMPLE_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT), 50);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(SAMPLE_INPUT), 0);
    // }
}