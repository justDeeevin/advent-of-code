const INPUT: &str = include_str!("../../input.txt");
// const INPUT: &str = r#"3-5
// 10-14
// 16-20
// 12-18
//
// 1
// 5
// 8
// 11
// 17
// 32"#;

type N = u64;

fn main() {
    let (ranges, numbers) = INPUT.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            start.parse::<N>().unwrap()..=end.parse::<N>().unwrap()
        })
        .collect::<Vec<_>>();

    let out = numbers.lines().fold(0, |acc, line| {
        if ranges
            .iter()
            .any(|range| range.contains(&line.parse::<N>().unwrap()))
        {
            acc + 1
        } else {
            acc
        }
    });

    dbg!(out);
}
