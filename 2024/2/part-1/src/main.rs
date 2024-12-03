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
            (*ascending.get_or_insert(diff > 0) == (diff > 0)) && (1..=3).contains(&diff.abs())
        });
        if safe { acc + 1 } else { acc }
    });

    println!("{}", out);
}
