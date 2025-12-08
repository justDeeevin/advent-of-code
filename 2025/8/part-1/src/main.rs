use ordered_float::OrderedFloat;
use petgraph::{
    data::{Element, FromElements},
    graph::{NodeIndex, UnGraph},
    unionfind::UnionFind,
    visit::{Bfs, VisitMap},
};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

const INPUT: &str = include_str!("../../input.txt");
const N_CONNECTIONS: usize = 1000;
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
// const N_CONNECTIONS: usize = 10;

type N = i64;

fn main() {
    let mut graph = UnGraph::<_, f64>::from_elements(INPUT.lines().map(|l| Element::Node {
        weight: Point::from(l),
    }));

    let n = graph.node_count();

    let mut uf = UnionFind::new(n);

    let mut edges = BinaryHeap::from_iter(
        graph
            .node_weights()
            .zip(graph.node_indices())
            .enumerate()
            .flat_map(|(i, (source, source_idx))| {
                graph
                    .node_weights()
                    .zip(graph.node_indices())
                    .skip(i + 1)
                    .map(move |(target, target_idx)| {
                        (
                            Reverse(OrderedFloat(source.distance_to(target))),
                            source_idx,
                            target_idx,
                        )
                    })
            }),
    );

    assert_eq!(edges.len(), (n * (n - 1)) / 2);

    for _ in 0..N_CONNECTIONS {
        let (Reverse(OrderedFloat(distance)), source, target) = edges.pop().unwrap();
        if !uf.equiv(source, target) {
            uf.union(source, target);
            graph.add_edge(source, target, distance);
        }
    }

    let mut groups = connected_groups(&graph);
    groups.sort_by_key(|group| Reverse(group.len()));

    let out = groups
        .iter()
        .take(3)
        .map(|group| group.len())
        .product::<usize>();
    dbg!(out);
}

fn connected_groups(graph: &UnGraph<Point, f64>) -> Vec<HashSet<NodeIndex>> {
    let mut visited = HashSet::new();
    let mut groups = Vec::new();

    for start in graph.node_indices() {
        if !visited.is_visited(&start) {
            let mut group = HashSet::new();
            let mut bfs = Bfs::new(&graph, start);

            while let Some(next) = bfs.next(&graph) {
                visited.visit(next);
                group.insert(next);
            }

            groups.push(group);
        }
    }

    groups
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
