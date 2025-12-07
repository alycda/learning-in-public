use day_07::part2;

fn main() {
    #[cfg(feature = "ci")]
    let file = day_07::SAMPLE_INPUT;

    #[cfg(not(feature = "ci"))]
    let file = include_str!("../../input.txt");

    let result = part2(file);
    println!("{}", result);
}