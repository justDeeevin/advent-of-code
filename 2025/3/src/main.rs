const INPUT: &str = include_str!("../input.txt");
// const INPUT: &str = r#"987654321111111
// 811111111111119
// 234234234234278
// 818181911112111"#;

type N = u64;

fn main() {
    let batteries = std::env::args()
        .nth(1)
        .expect("must provide number of batteries to use")
        .parse()
        .unwrap();
    let out = INPUT
        .trim()
        .lines()
        .map(|line| {
            let mut digits = Vec::<N>::with_capacity(batteries);
            let mut charges = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as N)
                .rev()
                .collect::<Vec<_>>();
            while let Some(charge) = charges.pop() {
                let digit = {
                    let mut start = None;

                    'a: loop {
                        for (i, d) in digits.iter().enumerate().skip(start.unwrap_or_default()) {
                            if start == Some(digits.len()) {
                                break;
                            }
                            if charge > *d {
                                if batteries - i <= charges.len() + 1 {
                                    break 'a Some(i);
                                } else {
                                    start = Some(i + 1);
                                    continue 'a;
                                }
                            }
                        }
                        break None;
                    }
                };
                if let Some(i) = digit {
                    digits.truncate(i + 1);
                    digits[i] = charge;
                } else if digits.len() < batteries {
                    digits.push(charge);
                }
            }

            digits
                .into_iter()
                .fold(String::new(), |acc, d| acc + &d.to_string())
                .parse::<N>()
                .unwrap()
        })
        .sum::<N>();

    dbg!(out);
}
