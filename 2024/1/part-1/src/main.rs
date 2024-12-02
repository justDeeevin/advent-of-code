type Id = u32;
type Count = i32;

fn main() {
    let deep: Vec<_> = include_str!("../../input.txt")
        .lines()
        .map(|line| line.split("   ").collect::<Vec<_>>())
        .collect();
    let mut left: Vec<_> = deep
        .iter()
        .map(|row| row[0].parse::<Id>().unwrap())
        .collect();
    let mut right: Vec<_> = deep
        .iter()
        .map(|row| row[1].parse::<Id>().unwrap())
        .collect();

    left.sort();
    right.sort();

    let total: Count = left
        .iter()
        .zip(right.iter())
        .map(|(left, right)| (*left as Count - *right as Count).abs())
        .sum();

    println!("{}", total);
}
