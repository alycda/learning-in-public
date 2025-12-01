fn part1(input: &str) -> Result<String, String> {
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

fn main() {
    let file = include_str!("../../input.txt");
    let result = part1(file);
    println!("{}", result.unwrap());
}