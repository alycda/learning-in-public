use day_09::part1;

fn main() {
    #[cfg(feature = "ci")]
    let file = day_09::SAMPLE_INPUT;

    #[cfg(not(feature = "ci"))]
    let file = include_str!("../../input.txt");

    let result = part1(file);
    println!("{}", result);
}