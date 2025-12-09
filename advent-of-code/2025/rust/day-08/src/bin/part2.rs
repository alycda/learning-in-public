use day_08::part2;

fn main() {
    #[cfg(feature = "ci")]
    let file = day_08::SAMPLE_INPUT;

    #[cfg(not(feature = "ci"))]
    let file = include_str!("../../input.txt");

    let result = part2(file);
    println!("{}", result);
}