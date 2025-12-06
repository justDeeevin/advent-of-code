use std::collections::HashSet;

// const INPUT: &str = include_str!("../../input.txt");
const INPUT: &str = r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#;

fn main() {
    let mut beams = HashSet::from([INPUT
        .lines()
        .next()
        .unwrap()
        .chars()
        .position(|c| c == 'S')
        .unwrap()]);

    let mut out = 0;

    for line in INPUT.lines().skip(1) {
        let splitters = line
            .chars()
            .enumerate()
            .filter(|(i, c)| *c == '^' && beams.contains(i))
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        out += splitters.len();

        for i in &splitters {
            beams.remove(i);
        }

        beams.extend(splitters.into_iter().flat_map(|i| [i - 1, i + 1]));
    }

    dbg!(out);
}
