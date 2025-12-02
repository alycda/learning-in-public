pub fn part1(input: &str) -> String {
    let mut invalid_ids = vec![0];

    input.lines().for_each(|line| {
        line.split(',')
            .for_each(|id| {
                // dbg!(&id);
                let mut ids = id.split('-');
                let a = ids.next().unwrap().parse::<u64>().unwrap();
                let b = ids.next().unwrap().parse::<u64>().unwrap();

                // dbg!(a, b);

                for i in a..=b {
                    let s = i.to_string();
                    let count = s.chars().count();

                    if count % 2 == 0 {
                        // dbg!(i);
                        let mid = count / 2;
                        let mut chars = s.chars();

                        if chars.by_ref().take(mid).collect::<Vec<_>>() == chars.by_ref().take(mid).collect::<Vec<_>>() {
                            // dbg!(i);
                            invalid_ids.push(i);
                        }
                    }
                }
            });
    });

    invalid_ids.iter().sum::<u64>().to_string()
}

fn part2(input: &str) -> String {
    let mut invalid_ids = vec![0];

    input.lines().for_each(|line| {
        line.split(',')
            .for_each(|id| {
                // dbg!(&id);
                let mut ids = id.split('-');
                let a = ids.next().unwrap().parse::<u64>().unwrap();
                let b = ids.next().unwrap().parse::<u64>().unwrap();

                // dbg!(a, b);

                for i in a..=b {
                    let s = i.to_string();
                    let count = s.chars().count();

                    // if count % 2 == 0 {
                        // dbg!(i);
                        let mid = count / 2;
                        let mut chars = s.chars();

                        if chars.by_ref().take(mid).collect::<Vec<_>>() == chars.by_ref().take(mid).collect::<Vec<_>>() {
                            dbg!(i);
                            invalid_ids.push(i);
                        }
                    // }
                }
            });
    });

    invalid_ids.iter().sum::<u64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"), "1227775554".to_string());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"), "4174379265".to_string());
    }
}