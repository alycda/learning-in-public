use day_01::Dial;

fn main() {
    let file = include_str!("../../input.txt");
    let result = Dial::part1(file);
    println!("{}", result.unwrap());
}