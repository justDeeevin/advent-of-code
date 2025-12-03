const INPUT: &str = include_str!("../../input.txt");
// const INPUT: &str = r#"987654321111111
// 811111111111119
// 234234234234278
// 818181911112111
// "#;

type N = u16;

fn main() {
    let out = INPUT
        .trim()
        .lines()
        .map(|line| {
            let mut first = None;
            let mut second = None;
            let mut chars = line.chars().peekable();
            while let Some(c) = chars.next() {
                let digit = c.to_digit(10).unwrap();
                if first.is_none_or(|n| chars.peek().is_some() && digit > n) {
                    first = Some(digit);
                    second = None;
                } else if second.is_none_or(|n| digit > n) {
                    second = Some(digit);
                }
            }

            format!("{}{}", first.unwrap(), second.unwrap())
                .parse::<N>()
                .unwrap()
        })
        .sum::<N>();

    dbg!(out);
}
