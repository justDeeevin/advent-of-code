use std::collections::BinaryHeap;

const INPUT: &str = include_str!("../../input.txt");
// const INPUT: &str = r#"7,1
// 11,1
// 11,7
// 9,7
// 9,5
// 2,5
// 2,3
// 7,3"#;

type N = i64;

fn main() {
    let points = INPUT.lines().map(Point::from).collect::<Vec<_>>();

    let mut areas = points
        .iter()
        .enumerate()
        .flat_map(|(i, p)| points.iter().skip(i + 1).map(|q| (q.area_with(p), *q, *p)))
        .collect::<BinaryHeap<_>>();

    let out = areas.pop().unwrap();

    dbg!(out);
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
struct Point {
    x: N,
    y: N,
}

impl Point {
    fn area_with(&self, other: &Point) -> N {
        ((self.x - other.x).abs() + 1) * ((self.y - other.y).abs() + 1)
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(',').unwrap();
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}
