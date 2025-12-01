use std::{num::{NonZeroI32, NonZeroU32, TryFromIntError}};

/// position, zero_crossings
pub struct Dial(i32, NonZeroU32);

impl std::ops::Deref for Dial {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Dial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", **self)
    }
}

enum Elf {
    /// left -
    Laverne(u32),
    /// right +
    Regina(u32),
}

impl From<&'static str> for Elf {
    fn from(line: &str) -> Self {
        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let distance = chars.collect::<String>().parse::<u32>().unwrap();

        match direction {
            'L' => { Self::Laverne(distance) },
            'R' => { Self::Regina(distance) },
            _ => unreachable!()
        }
    }
}

impl Dial {
    fn parse(input: &'static str) -> Vec<Elf> {
        input.lines().fold(vec![], |mut elves, line| {

            elves.push(Elf::from(line));

            elves
        })
    }

    /// The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence
    pub fn part1(input: &'static str) -> Result<NonZeroU32, TryFromIntError> {
        let mut zeros = 0;

        Self::parse(input)
            .into_iter()
            .fold(50_i32, |mut position, elf| {
                match elf {
                    Elf::Laverne(distance) => { position -= distance as i32; },
                    Elf::Regina(distance) => { position += distance as i32; },
                }

                position = position.rem_euclid(100);

                if position == 0 { zeros += 1 }

                position
            });
    
        NonZeroI32::new(zeros).unwrap().try_into()
    }

    fn part2() -> NonZeroU32 {
        todo!()
    }
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

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        assert_eq!(Dial::part1(INPUT), Ok(NonZeroU32::new(3).unwrap()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "6".to_string());
    }
}