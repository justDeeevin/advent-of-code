use std::collections::HashMap;

const INPUT: &str = include_str!("../../input.txt");
// const INPUT: &str = r#".......S.......
// ...............
// .......^.......
// ...............
// ......^.^......
// ...............
// .....^.^.^.....
// ...............
// ....^.^...^....
// ...............
// ...^.^...^.^...
// ...............
// ..^...^.....^..
// ...............
// .^.^.^.^.^...^.
// ..............."#;

fn main() {
    let mut beams = HashMap::from([(
        INPUT
            .lines()
            .next()
            .unwrap()
            .chars()
            .position(|c| c == 'S')
            .unwrap(),
        1,
    )]);

    for line in INPUT.lines().skip(1) {
        let mut next = beams.clone();

        for (col, count) in beams {
            if line.chars().nth(col).unwrap() == '^' {
                *next.entry(col - 1).or_default() += count;
                *next.entry(col + 1).or_default() += count;
                next.remove(&col);
            }
        }
        beams = next;
    }

    let out = beams.values().sum::<usize>();

    dbg!(out);
}
