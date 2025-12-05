use day_05::part2;

fn main() {
    #[cfg(feature = "ci")]
    let file = day_04::SAMPLE_INPUT;

    #[cfg(not(feature = "ci"))]
    let file = include_str!("../../input.txt");

    let result = part2(file);
    println!("{}", result);
}