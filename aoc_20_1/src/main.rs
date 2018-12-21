use std::collections::{HashSet, VecDeque};

fn main() {
    let map_size = 160;
    let start = Vec2::new(map_size / 2, map_size / 2);

    let mut map: Vec<Vec<Node>> = vec![vec![Node::new(); map_size]; map_size];
    let mut stack: VecDeque<Vec2> = VecDeque::new();
    let mut pos = start.clone();
    for c in include_str!("input.txt").chars() {
        match c {
            '^' => {
                map[pos.y][pos.x].free = true;
                map[pos.y][pos.x].distance = 0
            }
            '$' => (),
            'N' => {
                let npos = Vec2::new(pos.x, pos.y.wrapping_sub(1));
                insert_node(&mut map, &pos, &npos);
                pos = npos;
            }
            'S' => {
                let npos = Vec2::new(pos.x, pos.y + 1);
                insert_node(&mut map, &pos, &npos);
                pos = npos;
            }
            'E' => {
                let npos = Vec2::new(pos.x + 1, pos.y);
                insert_node(&mut map, &pos, &npos);
                pos = npos;
            }
            'W' => {
                let npos = Vec2::new(pos.x.wrapping_sub(1), pos.y);
                insert_node(&mut map, &pos, &npos);
                pos = npos;
            }
            '(' => stack.push_back(pos.clone()),
            ')' => pos = stack.pop_back().unwrap(),
            '|' => pos = stack.back().unwrap().clone(),
            _ => panic!("Unknown Symbol"),
        }
    }

    let mut result = 0;
    let mut result2 = 0;
    let mut bfs: VecDeque<Vec2> = VecDeque::new();
    bfs.push_back(start);
    while !bfs.is_empty() {
        let pos = bfs.pop_front().unwrap();
        let new_distance = map[pos.y][pos.x].distance + 1;
        for adj in map[pos.y][pos.x].adjacent.clone().iter() {
            if map[adj.y][adj.x].visited {
                continue;
            }
            map[adj.y][adj.x].distance = new_distance;
            map[adj.y][adj.x].visited = true;
            bfs.push_back(adj.clone());
        }
        result = result.max(map[pos.y][pos.x].distance);
        if map[pos.y][pos.x].distance >= 1000 {
            result2 += 1;
        }
    }
    println!("Result: {}", result);
    println!("Result 2: {}", result2);
}

fn insert_node(map: &mut Vec<Vec<Node>>, pos: &Vec2, npos: &Vec2) {
    map[npos.y][npos.x].free = true;
    map[pos.y][pos.x].adjacent.insert(npos.clone());
    map[npos.y][npos.x].adjacent.insert(pos.clone());
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Vec2 {
    x: usize,
    y: usize,
}

impl Vec2 {
    fn new(x: usize, y: usize) -> Self {
        Vec2 { x: x, y: y }
    }
}

#[derive(Debug, Clone)]
struct Node {
    distance: i32,
    adjacent: HashSet<Vec2>,
    visited: bool,
    free: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            distance: 99999,
            adjacent: HashSet::new(),
            visited: false,
            free: false,
        }
    }
}
