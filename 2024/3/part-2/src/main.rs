use regex::Regex;

fn main() {
    let input = include_str!("../../example_input.txt");
    let mul_regex = Regex::new(r"mul\(([0-9]*),([0-9]*)\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();
    let mul_captures = mul_regex
        .captures_iter(input)
        .map(|c| (c[1].parse::<u32>().unwrap(), c[2].parse::<u32>().unwrap()));
    let muls = mul_regex.find_iter(input);
    let mut dos = do_regex.find_iter(input).collect::<Vec<_>>();
    let donts = dont_regex.find_iter(input).collect::<Vec<_>>();
    let mut disabled_ranges = Vec::new();

    if dos[0].start() < donts[0].start() {
        dos.pop();
    }

    for dont_match in donts {
        if let Some(do_match) = dos.pop() {
            disabled_ranges.push(dont_match.start()..do_match.start());
        }
    }

    let out = muls
        .zip(mul_captures)
        .fold(0, |acc, (mul_match, (left, right))| {
            if disabled_ranges
                .iter()
                .any(|range| range.contains(&mul_match.start()))
            {
                acc
            } else {
                acc + (left * right)
            }
        });

    println!("{}", out);
}
