use day_01::Dial;

fn main() {
    #[cfg(feature = "ci")]
    let file = day_01::SAMPLE_INPUT;

    #[cfg(not(feature = "ci"))]
    let file = include_str!("../../input.txt");

    let result = Dial::part1(file);
    println!("{}", result.unwrap());
}
