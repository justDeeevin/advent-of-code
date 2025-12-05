matrix::matrix_file!(INPUT: "../input.txt");
// matrix::matrix_str!(INPUT: r#"..@@.@@@@.
// @@@.@.@.@@
// @@@@@.@.@@
// @.@@@@..@.
// @@.@@@@.@@
// .@@@@@@@.@
// .@.@.@.@@@
// @.@@@.@@@@
// .@@@@@@@@.
// @.@.@@@.@."#);

type N = i16;

fn main() {
    let width = INPUT[0].len();
    let out = INPUT
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
                    if INPUT[row - 1][(col as N + i) as usize] == '@' {
                        neighbors += 1;
                    }
                }
            }
            if col > 0 && INPUT[row][col - 1] == '@' {
                neighbors += 1;
            }
            if col < width - 1 && INPUT[row][col + 1] == '@' {
                neighbors += 1;
            }
            if row < INPUT.len() - 1 {
                for i in side_start..=side_end {
                    if INPUT[row + 1][(col as N + i) as usize] == '@' {
                        neighbors += 1;
                    }
                }
            }
            if neighbors < 4 { acc + 1 } else { acc }
        });
    dbg!(out);
}
