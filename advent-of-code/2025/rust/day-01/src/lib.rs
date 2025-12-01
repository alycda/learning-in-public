/// The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence
pub fn part1(input: &str) -> Result<String, String> {
    let mut start = 50_i32;
    let mut zeros = 0;

    input.lines().for_each(|line| {
        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let distance = chars.collect::<String>().parse::<i32>().unwrap();

        match direction {
            'L' => { start -= distance; },
            'R' => { start += distance; },
            _ => unreachable!()
        }

        start = start.rem_euclid(100);

        if start == 0 { zeros += 1 }
    });

    Ok(zeros.to_string())
}

/// password method 0x434C49434B
pub fn part2(input: &str) -> Result<String, String> {
    let mut start = 50_i32;
    let mut zeros = 0;

    input.lines().for_each(|line| {
        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let distance = chars.collect::<String>().parse::<i32>().unwrap();

        match direction {
            'L' => { 
                zeros += if start == 0 {
                    distance / 100
                } else if distance >= start {
                    1 + (distance - start) / 100
                } else {
                    0
                };

                start = (start - distance).rem_euclid(100);
            },
            'R' => {  
                zeros += (start + distance) / 100;
                start = (start + distance) % 100;
            },
            _ => unreachable!()
        }
    });

    Ok(zeros.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(part1(input).unwrap(), "3".to_string());
    }

    #[test]
    fn test_part2() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(part2(input).unwrap(), "6".to_string());
    }
}