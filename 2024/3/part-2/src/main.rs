use regex::Regex;
use std::collections::VecDeque;
use std::ops::Range;

fn main() {
    let input = include_str!("../../input.txt");
    let mul_regex = Regex::new(r"mul\(([0-9]*),([0-9]*)\)").unwrap();
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();
    let mul_captures = mul_regex
        .captures_iter(input)
        .map(|c| (c[1].parse::<u32>().unwrap(), c[2].parse::<u32>().unwrap()));
    let muls = mul_regex.find_iter(input);
    let mut dos = do_regex.find_iter(input).collect::<VecDeque<_>>();
    let donts = dont_regex.find_iter(input).collect::<Vec<_>>();
    let mut disabled_ranges = Vec::new();

    for dont_match in donts {
        if disabled_ranges
            .iter()
            .any(|r: &Range<usize>| r.contains(&dont_match.start()))
        {
            continue;
        }

        if dos.is_empty() {
            disabled_ranges.push(dont_match.start()..input.len());
        }
        while let Some(do_match) = dos.pop_front() {
            if do_match.start() < dont_match.start() {
                continue;
            } else {
                disabled_ranges.push(dont_match.start()..do_match.start());
                break;
            }
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
