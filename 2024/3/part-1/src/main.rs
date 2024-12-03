use regex::Regex;

fn main() {
    let input = include_str!("../../input.txt");
    let regex = Regex::new(r"mul\(([0-9]*),([0-9]*)\)").unwrap();
    let out = regex
        .captures_iter(input)
        .map(|c| (c[1].parse::<u32>().unwrap(), c[2].parse::<u32>().unwrap()))
        .fold(0, |acc, (left, right)| acc + (left * right));

    println!("{}", out);
}
