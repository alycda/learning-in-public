use day_01::part2;

fn main() {
    let file = include_str!("../../input.txt");
    let result = part2(file);
    // let result = Dial.spin(file).1;
    println!("{}", result.unwrap());
}