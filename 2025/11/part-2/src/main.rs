use petgraph::{Graph, graph::NodeIndex};
use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../../input.txt");
// const INPUT: &str = r#"svr: aaa bbb
// aaa: fft
// fft: ccc
// bbb: tty
// tty: ccc
// ccc: ddd eee
// ddd: hub
// hub: fff
// eee: dac
// dac: fff
// fff: ggg hhh
// ggg: out
// hhh: out"#;

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

    let svr_idx = *indices.get("svr").unwrap();
    let out_idx = *indices.get("out").unwrap();
    let fft_idx = *indices.get("fft").unwrap();
    let dac_idx = *indices.get("dac").unwrap();

    let svr_to_fft = count_paths(&graph, svr_idx, fft_idx);
    let svr_to_dac = count_paths(&graph, svr_idx, dac_idx);
    let fft_to_dac = count_paths(&graph, fft_idx, dac_idx);
    let fft_to_out = count_paths(&graph, fft_idx, out_idx);
    let dac_to_fft = count_paths(&graph, dac_idx, fft_idx);
    let dac_to_out = count_paths(&graph, dac_idx, out_idx);

    let out = (svr_to_fft * fft_to_dac * dac_to_out) + (svr_to_dac * dac_to_fft * fft_to_out);

    dbg!(out);
}

fn count_paths(graph: &Graph<&str, ()>, src: NodeIndex, dest: NodeIndex) -> usize {
    fn dfs(
        graph: &Graph<&str, ()>,
        node: NodeIndex,
        dest: NodeIndex,
        memo: &mut HashMap<NodeIndex, usize>,
        visited: &mut HashSet<NodeIndex>,
    ) -> usize {
        if node == dest {
            return 1;
        }
        if visited.contains(&node) {
            return 0;
        }
        if let Some(cached) = memo.get(&node) {
            return *cached;
        }

        visited.insert(node);
        let total = graph
            .neighbors(node)
            .map(|neighbor| dfs(graph, neighbor, dest, memo, visited))
            .sum();
        visited.remove(&node);
        memo.insert(node, total);
        total
    }
    dfs(graph, src, dest, &mut HashMap::new(), &mut HashSet::new())
}
