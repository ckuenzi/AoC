use std::collections::{HashMap, HashSet};

fn main() {
    let mut nodes = HashSet::new();
    let mut dependencies = include_str!("input.txt")
        .lines()
        .map(|line| (line.chars().nth(5).unwrap(), line.chars().nth(36).unwrap()))
        .fold(HashMap::new(), |mut map, dep| {
            map.entry(dep.1).or_insert(vec![]).push(dep.0);
            map.entry(dep.0).or_insert(vec![]);
            nodes.insert(dep.0);
            nodes.insert(dep.1);
            map
        });
    let mut nodes: Vec<_> = nodes.iter().collect();
    nodes.sort();

    while nodes.len() > 0 {
        for i in 0..nodes.len() {
            if dependencies.get(nodes[i]).unwrap().len() == 0 {
                print!("{}", nodes[i]);
                dependencies.remove(nodes[i]);
                for j in 0..nodes.len() {
                    if let Some(list) = dependencies.get_mut(nodes[j]) {
                        if let Some(index) = list.iter().position(|n| n == nodes[i]) {
                            list.remove(index);
                        }
                    }
                }
                nodes.remove(i);
                break;
            }
        }
    }
}
