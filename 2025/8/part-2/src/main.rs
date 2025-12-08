use ordered_float::OrderedFloat;
use petgraph::unionfind::UnionFind;
use std::{cmp::Reverse, collections::BinaryHeap};

const INPUT: &str = include_str!("../../input.txt");
// const INPUT: &str = r#"162,817,812
// 57,618,57
// 906,360,560
// 592,479,940
// 352,342,300
// 466,668,158
// 542,29,236
// 431,825,988
// 739,650,466
// 52,470,668
// 216,146,977
// 819,987,18
// 117,168,530
// 805,96,715
// 346,949,466
// 970,615,88
// 941,993,340
// 862,61,35
// 984,92,344
// 425,690,689"#;

type N = i64;

fn main() {
    let points = INPUT.lines().map(Point::from).collect::<Vec<_>>();
    let n = points.len();

    let mut uf = UnionFind::new(n);

    let mut edges =
        BinaryHeap::from_iter(points.iter().enumerate().flat_map(|(source_idx, source)| {
            points
                .iter()
                .enumerate()
                .skip(source_idx + 1)
                .map(move |(target_idx, target)| {
                    (
                        Reverse(OrderedFloat(source.distance_to(target))),
                        source_idx,
                        target_idx,
                    )
                })
        }));

    assert_eq!(edges.len(), (n * (n - 1)) / 2);

    let mut out = 0;

    while let Some((_, source, target)) = edges.pop() {
        if !uf.equiv(source, target) {
            uf.union(source, target);
            out = points[source].x * points[target].x;
        }
    }

    assert!((0..points.len()).all(|i| (i..points.len()).all(|j| uf.equiv(i, j))));

    dbg!(out);
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: N,
    y: N,
    z: N,
}

impl Default for Point {
    fn default() -> Self {
        panic!("Point::default() should not be called")
    }
}

impl Point {
    fn distance_to(&self, other: &Point) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .sqrt()
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let mut parts = value.split(',');
        Point {
            x: parts.next().unwrap().parse().unwrap(),
            y: parts.next().unwrap().parse().unwrap(),
            z: parts.next().unwrap().parse().unwrap(),
        }
    }
}
