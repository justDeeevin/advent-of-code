fn main() {
    let reports = include_str!("../../input.txt")
        .lines()
        .map(|l| {
            l.split(" ")
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let out = reports.iter().fold(0, |acc, report| {
        let mut ascending = None;
        let safe = report.windows(2).all(|window| {
            let diff = window[1] as i8 - window[0] as i8;
            let small_diff = (1..=3).contains(&diff.abs());
            match ascending {
                None => {
                    ascending = Some(diff > 0);
                    small_diff
                }
                Some(ascending) => (ascending == (diff > 0)) && small_diff,
            }
        });
        if safe { acc + 1 } else { acc }
    });

    println!("{}", out);
}
