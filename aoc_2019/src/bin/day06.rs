use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

fn main() {
    let (system, nodes) = parse_input();
    let from_com = dijkstra(&system, *nodes.get("COM").unwrap(), None, |_| 1);
    let weights = from_com.iter().map(|n| n.1);
    let mut part1 = 0;
    for weight in weights {
        part1 += weight;
    }

    println!(
        "{}\n{}",
        part1,
        dijkstra(&system, *nodes.get("YOU").unwrap(), None, |_| 1)
            .get(nodes.get("SAN").unwrap())
            .unwrap()
            - 2
    );
}

fn parse_input() -> (Graph<u32, u32>, HashMap<String, NodeIndex>) {
    let mut system = Graph::new();
    let inputs = include_str!("inputs\\day06a.txt")
        .lines()
        .map(|l| l.split(')').collect::<Vec<_>>());

    let mut nodes = HashMap::<String, NodeIndex>::new();
    for n in inputs.clone().flatten().unique() {
        nodes.insert(n.to_string(), system.add_node(hash_str(n)));
    }
    for e in inputs {
        system.add_edge(*nodes.get(e[0]).unwrap(), *nodes.get(e[1]).unwrap(), 1);
        system.add_edge(*nodes.get(e[1]).unwrap(), *nodes.get(e[0]).unwrap(), 1);
    }
    (system, nodes)
}

fn hash_str(input: &str) -> u32 {
    let mut s = DefaultHasher::new();
    input.hash(&mut s);
    s.finish() as u32
}
