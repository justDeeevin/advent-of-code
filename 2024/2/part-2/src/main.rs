fn main() {
    let reports = include_str!("../../input.txt")
        .lines()
        .map(|l| {
            l.split(" ")
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let out = reports
        .iter()
        .enumerate()
        .fold(0, |acc, (report_index, report)| {
            let mut ascending = None;
            let mut dampened = false;
            let safe = report.windows(3).enumerate().all(|(level_index, window)| {
                let diff = window[1] as i8 - window[0] as i8;
                let safe = is_diff_safe(diff, &mut ascending);
                if !safe
                    && !dampened
                    && is_diff_safe(window[2] as i8 - window[0] as i8, &mut ascending)
                {
                    print!(
                        "Bad level {} at index {} of report {:?} dampened.",
                        window[1],
                        level_index + 1,
                        report
                    );
                    dampened = true;
                    true
                } else {
                    safe
                }
            });

            let last_two = report.last_chunk::<2>().unwrap();

            if !is_diff_safe(last_two[1] as i8 - last_two[0] as i8, &mut ascending) {
                if dampened {
                    acc
                } else {
                    print!(
                        "Bad level {} at index {} of report {:?} (No. {}) dampened.",
                        last_two[1],
                        report.len() - 1,
                        report,
                        report_index + 1,
                    );
                    acc + 1
                }
            } else if safe {
                acc + 1
            } else {
                acc
            }
            println!();
        });

    println!("{}", out);
}

fn is_diff_safe(diff: i8, ascending: &mut Option<bool>) -> bool {
    (*ascending.get_or_insert(diff > 0) == (diff > 0)) && (1..=3).contains(&diff.abs())
}
