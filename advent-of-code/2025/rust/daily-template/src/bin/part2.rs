use {{crate_name}}::part2;

fn main() {
    #[cfg(feature = "ci")]
    let file = {{crate_name}}::SAMPLE_INPUT;

    #[cfg(not(feature = "ci"))]
    let file = include_str!("../../input.txt");

    let result = part2(file);
    println!("{}", result);
}