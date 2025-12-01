use std::{num::{NonZeroI32, NonZeroU32, TryFromIntError}};

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
    fn current_position(&self) -> u32 {
        **self as u32
    }

    fn zero_crossings(&self) -> u32 {
        self.1
    }

    fn parse(input: &'static str) -> Rotations {
        input.lines().fold(vec![], |mut elves, line| {

            elves.push(Elf::from(line));

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
                    position = ffi::rem_euclid(position, 100)
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

#[cfg(feature = "ffi_c")]
pub mod ffi {
    // use super::*;

    /* 
    error: linking with `cc` failed: exit status: 1                                                                                                                ▐                                         ▐
   |                                                                                                                                                               ▐                                         ▐
   = note:  "cc" "/private/tmp/nix-shell-85922-0/rustcrdrgGg/symbols.o" "<53 object files omitted>" "-lmath" "<sysroot>/lib/rustlib/aarch64-apple-darwin/lib/{libte▐                                         ▐
 st-*,libgetopts-*,librustc_std_workspace_std-*,libstd-*,libpanic_unwind-*,libobject-*,libmemchr-*,libaddr2line-*,libgimli-*,libcfg_if-*,librustc_demangle-*,libstd▐                                         ▐
 _detect-*,libhashbrown-*,librustc_std_workspace_alloc-*,libminiz_oxide-*,libadler2-*,libunwind-*,liblibc-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libc▐                                         ▐
 ompiler_builtins-*}.rlib" "-lSystem" "-lc" "-lm" "-arch" "arm64" "-mmacosx-version-min=11.3.0" "-o" "/Users/alyssaevans/Work/learning-in-public/advent-of-code/202▐                                         ▐
 5/rust/target/debug/deps/day_01-d8a32a5b7a34b3ff" "-Wl,-dead_strip" "-nodefaultlibs"                                                                              ▐
   = note: some arguments are omitted. use `--verbose` to show all linker arguments                                                                                ▐
   = note: ld: library not found for -lmath
           clang: error: linker command failed with exit code 1 (use -v to see invocation)
    */ 
    // #[link(name = "math")] 
    // #[link(name = "m")] // Links with the compiled C library (libmath.a or libmath.so)
    // https://doc.rust-lang.org/nomicon/ffi.html#calling-foreign-functions
    // https://doc.rust-lang.org/nomicon/ffi.html#linking
    unsafe extern "C" {
        pub fn rem_euclid(a: i32, b: i32) -> i32;
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn ffi_part1() {
            assert_eq!(unsafe { rem_euclid(101,100) }, 101_i32.rem_euclid(100));
            assert_eq!(unsafe { rem_euclid(201,100) }, 201_i32.rem_euclid(100));
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