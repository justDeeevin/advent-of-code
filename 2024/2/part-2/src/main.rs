use colored::*;

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
            match first_bad_window(report) {
                None => acc + 1,
                Some(bad_window) => {
                    let mut trial_report = report.clone();
                    trial_report.remove(bad_window + 1);
                    if (first_bad_window(&trial_report)).is_none() {
                        alert_of_dampen(report, bad_window + 1, report_index);
                        acc + 1
                    } else {
                        let mut trial_report = report.clone();
                        trial_report.remove(bad_window);
                        if (first_bad_window(&trial_report)).is_none() {
                            alert_of_dampen(report, bad_window, report_index);
                            acc + 1
                        } else {
                            acc
                        }
                    }
                }
            }
        });

    println!("{}", out);
}

fn first_bad_window(report: &[u8]) -> Option<usize> {
    let mut ascending = None;
    for (left_i, window) in report.windows(2).enumerate() {
        let diff = window[1] as i8 - window[0] as i8;
        if !((*ascending.get_or_insert(diff > 0) == (diff > 0)) && (1..=3).contains(&diff.abs())) {
            return Some(left_i);
        }
    }
    None
}

fn alert_of_dampen(report: &[u8], removed_index: usize, report_index: usize) {
    let mut string = "[".to_string();
    for (i, level) in report.iter().take(report.len() - 1).enumerate() {
        let level = if i == removed_index {
            level.to_string().red().to_string()
        } else {
            level.to_string()
        };
        string.push_str(&format!("{}, ", level));
    }
    let last_level = if removed_index == report.len() - 1 {
        report.last().unwrap().to_string().red().to_string()
    } else {
        report.last().unwrap().to_string()
    };
    string.push_str(&format!("{}]", last_level));
    println!("Successfully dampened report {}: {}", report_index, string);
}
