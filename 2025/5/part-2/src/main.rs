use btree_range_map::RangeSet;

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
    let ranges = INPUT
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            start.parse::<N>().unwrap()..=end.parse::<N>().unwrap()
        })
        .collect::<RangeSet<_>>();

    let out = ranges.iter().fold(0, |acc, r| acc + r.len());

    dbg!(out);
}
