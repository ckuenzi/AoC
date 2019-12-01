//big oof
use std::collections::{HashMap, HashSet, VecDeque};
#[derive(Copy, Clone)]
struct Elve {
    node: char,
    finish_time: i32,
}

fn main() {
    let mut free_elves = 5;
    let tpt = 60;
    let mut to_remove: VecDeque<char> = VecDeque::new();
    let mut busy_elves: Vec<Elve> = vec![];

    let mut nodes: HashSet<char> = HashSet::new();
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

    let mut time = -1;

    while nodes.len() > 0 {
        time += 1;

        for i in 0..nodes.len() {
            if dependencies.get(nodes[i]).unwrap().len() == 0
                && !to_remove.contains(nodes[i])
                && !busy_elves
                    .iter()
                    .map(|e| e.node)
                    .collect::<Vec<_>>()
                    .contains(nodes[i])
            {
                to_remove.push_back(*nodes[i]);
            }
        }

        let mut removed_elve_offset: usize = 0;
        for i in 0..busy_elves.len() {
            let i = i.wrapping_sub(removed_elve_offset);
            if i > 1000 {
                continue;
            }

            if busy_elves[i].finish_time == time {
                let node = busy_elves[i].node;
                for j in 0..nodes.len() {
                    if let Some(list) = dependencies.get_mut(nodes[j]) {
                        if let Some(index) = list.iter().position(|n| *n == node) {
                            list.remove(index);
                        } else {
                        }
                    }
                }
                nodes = nodes.iter().fold(vec![], |mut out, c| {
                    if **c != node {
                        out.push(c);
                    }
                    out
                });
            }
            busy_elves = busy_elves.iter().fold(vec![], |mut out, elve| {
                if elve.finish_time >= time - 1 {
                    out.push(*elve);
                } else {
                    free_elves += 1;
                    removed_elve_offset += 1;
                }
                out
            });
        }

        while to_remove.len() > 0 && free_elves > 0 {
            free_elves -= 1;
            let node = to_remove.pop_front().unwrap();
            let new_elve = Elve {
                node: node,
                finish_time: time + tpt + ((node as u8 - b'@') as i32) - 1,
            };
            busy_elves.push(new_elve);
        }
    }
    println!("{}", time);
}
