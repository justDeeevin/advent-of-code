use std::collections::HashMap;

type Id = u32;
type Count = u32;

fn main() {
    let deep: Vec<_> = include_str!("../../input.txt")
        .lines()
        .map(|line| line.split("   ").collect::<Vec<_>>())
        .collect();
    let left: Vec<_> = deep
        .iter()
        .map(|row| row[0].parse::<Id>().unwrap())
        .collect();
    let right: Vec<_> = deep
        .iter()
        .map(|row| row[1].parse::<Id>().unwrap())
        .collect();

    let mut appearences: HashMap<Id, Count> = HashMap::new();

    for id in right {
        appearences
            .entry(id)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let similarity: Count = left
        .iter()
        .map(|id| *id * *appearences.entry(*id).or_default())
        .sum();

    println!("{similarity}");
}
