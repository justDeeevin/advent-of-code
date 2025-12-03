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
                    let mut pattern = String::new();

                    let mut chars = string.chars();
                    loop {
                        let Some(c) = chars.next() else {
                            return false;
                        };
                        if !pattern.starts_with(c) {
                            pattern.push(c);
                        } else {
                            break;
                        }
                    }

                    string.len() % pattern.len() == 0
                        && string
                            .as_bytes()
                            .chunks(pattern.len())
                            .map(|b| unsafe { str::from_utf8_unchecked(b) })
                            .all(|s| s == pattern)
                })
                .sum::<N>()
        });
    dbg!(out);
}
