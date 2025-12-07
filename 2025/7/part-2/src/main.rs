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
        for (col, count) in beams.clone() {
            if line.chars().nth(col).unwrap() == '^' {
                *beams.entry(col - 1).or_default() += count;
                *beams.entry(col + 1).or_default() += count;
                beams.remove(&col);
            }
        }
    }

    let out = beams.values().sum::<usize>();

    dbg!(out);
}
