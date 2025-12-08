use glam::IVec3;

// pub fn manhattan_distance(a: &IVec3, b: &IVec3) -> i32 {
//     (a.x - b.x).abs() + (a.y - b.y).abs()
// }

pub fn part1(input: &str) -> usize {
    let junction_boxes = input.lines().fold(vec![], |mut acc, line| {
        acc.push(IVec3::from_slice(&line.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>()));

        acc
    });

    dbg!(junction_boxes);

    // dbg!(IVec3::new(57,618,57).manhattan_distance(IVec3::new(162,817,812)));

    todo!()
}

pub fn part2(input: &str) -> usize {
    todo!()
}

pub const SAMPLE_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE_INPUT), 40);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(SAMPLE_INPUT), 0);
    // }
}