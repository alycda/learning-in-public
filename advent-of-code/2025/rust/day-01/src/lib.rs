use std::{num::{NonZeroI32, NonZeroU32, TryFromIntError}, str::FromStr};

/// Sample input for CI (real input.txt is gitignored)
pub const SAMPLE_INPUT: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

type Rotations = Vec<Elf>;
type Solution = NonZeroU32;
/// part1, part2
type Solutions = (NonZeroU32, NonZeroU32);

/// position, zero_crossings
pub struct Dial(i32, u32);

impl std::ops::Deref for Dial {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Dial {
    fn default() -> Self {
        Self(50, u32::default())
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

impl FromStr for Elf {
    type Err = String;

    fn from_str(line: &str) -> Result<Elf, String> {
        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        let distance = chars.collect::<String>().parse::<u32>().unwrap();

        match direction {
            'L' => { Ok(Self::Laverne(distance)) },
            'R' => { Ok(Self::Regina(distance)) },
            _ => unreachable!()
        }
    }
}

impl Dial {
    fn current_position(&self) -> u32 {
        **self as u32
    }

    fn zero_crossings(&self) -> u32 {
        self.1
    }

    fn parse(input: &'static str) -> Rotations {
        input.lines().fold(vec![], |mut elves, line| {

            elves.push(Elf::from_str(line).unwrap());

            elves
        })
    }

    /// The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence
    pub fn part1(input: &'static str) -> Result<Solution, TryFromIntError> {
        let mut zeros = 0;

        Self::parse(input)
            .into_iter()
            .fold(50_i32, |mut position, elf| {
                match elf {
                    Elf::Laverne(distance) => { position -= distance as i32; },
                    Elf::Regina(distance) => { position += distance as i32; },
                }

                #[cfg(not(feature = "ffi_c"))] 
                {
                    position = position.rem_euclid(100);
                }

                #[cfg(feature = "ffi_c")]
                unsafe {
                    let rust_rem_euclid = position.rem_euclid(100);
                    position = ffi::euclidean_remainder(position, 100);
                    assert_eq!(position, rust_rem_euclid);
                }

                if position == 0 { zeros += 1 }

                position
            });
    
        NonZeroI32::new(zeros).unwrap().try_into()
    }

    /// -> (part1, part2)
    fn spin(&mut self, elves: Rotations) -> Solutions {
        let mut lands_on_zeros = 0;
        let mut crosses_zeros = 0;

        elves.into_iter().for_each(|elf| {
            todo!();

            **self = 1000;
        });
        
        // todo: itertools::collect_tuple()
        (NonZeroI32::new(lands_on_zeros).unwrap().try_into().unwrap(), NonZeroI32::new(crosses_zeros).unwrap().try_into().unwrap())
    }

    fn part2() -> Solution {
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

                #[cfg(not(feature = "ffi_c"))] 
                {
                    start = (start - distance).rem_euclid(100);
                }

                #[cfg(feature = "ffi_c")]
                unsafe {
                    let rust_rem_euclid = (start - distance).rem_euclid(100);
                    start = ffi::euclidean_remainder((start - distance), 100);
                    assert_eq!(start, rust_rem_euclid);
                }
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

#[cfg(feature = "ffi_c")]
pub mod ffi {
    unsafe extern "C" {
        pub fn euclidean_remainder(a: i32, b: i32) -> i32;
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use rstest::rstest;

        #[rstest]
        #[case(101, 1)]
        #[case(201, 1)]
        fn ffi_part1(#[case] input: i32,#[case] expected: i32) {
            assert_eq!(unsafe { euclidean_remainder(input,100) }, expected);
            // assert_eq!(unsafe { euclidean_remainder(101,100) }, 101_i32.rem_euclid(100));
            // assert_eq!(unsafe { euclidean_remainder(201,100) }, 201_i32.rem_euclid(100));
        }
    }
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
        // assert_eq!(Dial::default().spin(Dial::parse(INPUT)).zero_crossings());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "6".to_string());
        // assert_eq!(Dial::part2(INPUT), Ok(NonZeroU32::new(6).unwrap()));
    }
}