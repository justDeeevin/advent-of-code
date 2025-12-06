// const INPUT: &str = include_str!("../../input.txt");
const INPUT: &str = r#"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "#;

type N = i64;

fn main() {
    let lines = INPUT
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();
    let pre_matrix = lines.into_iter().flatten().collect::<Vec<_>>();
    let mut matrix = vec![""; pre_matrix.len()];
    transpose::transpose(&pre_matrix, &mut matrix, width, height);
    let matrix = matrix.chunks(height).collect::<Vec<_>>();
    let out = matrix.into_iter().fold(0, |acc, row| {
        let op = row.last().unwrap().trim();
        let mut values = Vec::<String>::new();
        for value in row.iter().take(row.len() - 1) {
            for (i, v) in value.chars().enumerate() {
                if v == ' ' {
                    continue;
                }
                if let Some(s) = values.get_mut(i) {
                    s.push(v);
                } else {
                    values.push(v.into());
                }
            }
        }
        match op {
            "+" => {
                acc + values
                    .into_iter()
                    .map(|s| s.parse::<N>().unwrap())
                    .sum::<N>()
            }
            "*" => {
                acc + values
                    .into_iter()
                    .map(|s| s.parse::<N>().unwrap())
                    .product::<N>()
            }
            _ => unreachable!(),
        }
    });
    dbg!(out);
}
