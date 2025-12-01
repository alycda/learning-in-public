fn part2(input: &str) -> Result<String, String> {
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

fn main() {
    let file = include_str!("../../input.txt");
    let result = part2(file);
    println!("{}", result.unwrap());
}