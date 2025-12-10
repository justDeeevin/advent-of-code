use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("../../input.txt");
// const INPUT: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
// [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

type N = u16;

fn main() {
    let out = INPUT.lines().map(|l| handle_line(l).unwrap()).sum::<N>();

    dbg!(out);
}

fn handle_line(line: &str) -> Option<N> {
    let mut parts = line.split(' ');

    let dest = parts
        .next()?
        .strip_prefix('[')?
        .strip_suffix(']')?
        .chars()
        .enumerate()
        .fold(0, |acc, (i, c)| match c {
            '.' => acc,
            '#' => acc | 1 << i,
            _ => unreachable!(),
        });

    let buttons = parts
        .filter_map(|s| {
            Some(
                s.strip_prefix('(')?
                    .strip_suffix(')')?
                    .split(',')
                    .map(|n| n.parse::<N>().unwrap())
                    .fold(0, |acc, n| acc | 1 << n),
            )
        })
        .collect::<Vec<_>>();

    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    let mut visited = HashSet::new();

    while let Some((state, dist)) = queue.pop_front() {
        if state == dest {
            return Some(dist);
        }

        for button in &buttons {
            let next = state ^ button;
            if visited.insert(next) {
                queue.push_back((next, dist + 1));
            }
        }
    }

    None
}
