use itertools::Itertools;
use petgraph::algo::dijkstra;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[aoc_generator(day6)]
fn parse_input(input: &str) -> (Graph<u32, u32>, HashMap<String, NodeIndex>) {
    let mut system = Graph::new();
    let inputs = input.lines().map(|l| l.split(')').collect::<Vec<_>>());

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

#[aoc(day6, part1)]
fn part1(inputs: &(Graph<u32, u32>, HashMap<String, NodeIndex>)) -> i32 {
    let (system, nodes) = inputs;
    let from_com = dijkstra(&system, *nodes.get("COM").unwrap(), None, |_| 1);
    let weights = from_com.iter().map(|n| n.1);
    let mut part1 = 0;
    for weight in weights {
        part1 += *weight;
    }
    part1
}

#[aoc(day6, part2)]
fn part2(inputs: &(Graph<u32, u32>, HashMap<String, NodeIndex>)) -> i32 {
    let (system, nodes) = inputs;
    dijkstra(&system, *nodes.get("YOU").unwrap(), None, |_| 1)
        .get(nodes.get("SAN").unwrap())
        .unwrap()
        - 2
}

fn hash_str(input: &str) -> u32 {
    let mut s = DefaultHasher::new();
    input.hash(&mut s);
    s.finish() as u32
}
