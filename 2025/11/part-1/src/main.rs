use petgraph::{Graph, algo::all_simple_paths};
use std::{
    collections::HashMap,
    hash::{BuildHasherDefault, DefaultHasher},
};

const INPUT: &str = include_str!("../../input.txt");
// const INPUT: &str = r#"aaa: you hhh
// you: bbb ccc
// bbb: ddd eee
// ccc: ddd eee fff
// ddd: ggg
// eee: out
// fff: out
// ggg: out
// hhh: ccc fff iii
// iii: out"#;

fn main() {
    let mut graph = Graph::new();
    let mut indices = HashMap::new();

    for line in INPUT.lines() {
        let (src, dests) = line.split_once(": ").unwrap();
        let src_idx = *indices.entry(src).or_insert_with(|| graph.add_node(src));
        for dest in dests.split_whitespace() {
            let dest_idx = *indices.entry(dest).or_insert_with(|| graph.add_node(dest));
            graph.add_edge(src_idx, dest_idx, ());
        }
    }
    let out = all_simple_paths::<Vec<_>, _, BuildHasherDefault<DefaultHasher>>(
        &graph,
        *indices.get("you").unwrap(),
        *indices.get("out").unwrap(),
        1,
        None,
    )
    .count();

    dbg!(out);
}
