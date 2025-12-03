const INPUT: &str = include_str!("../../input.txt");

type N = u64;

fn main() {
    let out = INPUT
        .trim()
        .split(',')
        .map(|s| {
            let [left, right] = s
                .split('-')
                .map(|s| s.parse::<N>().unwrap())
                .collect::<Vec<_>>()[..]
            else {
                unreachable!()
            };
            left..=right
        })
        .fold(0, |acc, range| {
            acc + range
                .filter(|n| {
                    let string = n.to_string();
                    if string.len() % 2 != 0 {
                        return false;
                    }
                    let (left, right) = string.split_at(string.len() / 2);
                    left.parse::<N>().unwrap() == right.parse::<N>().unwrap()
                })
                .sum::<N>()
        });

    dbg!(out);
}
