use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt")
        .split("\n\n")
        .collect::<Vec<_>>();

    let rules = input[0]
        .lines()
        .map(|line| {
            let parts = line.split('|').collect::<Vec<_>>();
            (
                parts[0].parse::<u8>().unwrap(),
                parts[1].parse::<u8>().unwrap(),
            )
        })
        .fold(HashMap::new(), |mut acc, (page, follower)| {
            acc.entry(page)
                .and_modify(|list: &mut Vec<u8>| list.push(follower))
                .or_insert(vec![follower]);
            acc
        });

    let updates = input[1]
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let out = updates.iter().fold(0, |acc, update| {
        if !valid_update(&rules, update) {
            acc
        } else {
            acc + update[update.len() / 2] as u16
        }
    });

    println!("{out}");
}

fn valid_update(rules: &HashMap<u8, Vec<u8>>, update: &[u8]) -> bool {
    let mut past = Vec::new();
    !update.iter().any(|page| {
        past.push(page);
        let Some(followers) = rules.get(page) else {
            return false;
        };
        followers.iter().any(|p| past.contains(&p))
    })
}
