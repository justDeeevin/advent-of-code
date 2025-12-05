use std::collections::HashSet;

type N = i16;

fn main() {
    matrix::mut_matrix_file!(input: "../input.txt");
    // matrix::mut_matrix_str!(input: r#"..@@.@@@@.
    // @@@.@.@.@@
    // @@@@@.@.@@
    // @.@@@@..@.
    // @@.@@@@.@@
    // .@@@@@@@.@
    // .@.@.@.@@@
    // @.@@@.@@@@
    // .@@@@@@@@.
    // @.@.@@@.@."#);

    let width = input[0].len();
    let mut out = 0;
    loop {
        let mut removed = HashSet::new();
        let add = input
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(col, val)| (*val == '@').then_some((i, col)))
            })
            .fold(0, |acc, (row, col)| {
                let side_start = if col > 0 { -1 } else { 0 };
                let side_end = if col < width - 1 { 1 } else { 0 };
                let mut neighbors = 0;
                if row > 0 {
                    for i in side_start..=side_end {
                        let index = (row - 1, (col as N + i) as usize);
                        if input[index.0][index.1] == '@' {
                            neighbors += 1;
                        }
                    }
                }
                if col > 0 && input[row][col - 1] == '@' {
                    neighbors += 1;
                }
                if col < width - 1 && input[row][col + 1] == '@' {
                    neighbors += 1;
                }
                if row < input.len() - 1 {
                    for i in side_start..=side_end {
                        let index = (row + 1, (col as N + i) as usize);
                        if input[index.0][index.1] == '@' {
                            neighbors += 1;
                        }
                    }
                }
                if neighbors < 4 {
                    removed.insert((row, col));
                    acc + 1
                } else {
                    acc
                }
            });
        if add == 0 {
            break;
        } else {
            for (row, col) in removed {
                assert_eq!(input[row][col], '@', "non-full cell was marked for removal");
                input[row][col] = '.';
            }
            out += add;
        }
    }
    dbg!(out);
}
