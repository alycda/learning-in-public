use {{crate_name}}::part1;

fn main() {
    #[cfg(feature = "ci")]
    let file = {{crate_name}}::SAMPLE_INPUT;

    #[cfg(not(feature = "ci"))]
    let file = include_str!("../../input.txt");

    let result = part1(file);
    println!("{}", result);
}