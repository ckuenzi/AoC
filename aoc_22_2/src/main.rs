use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt")
        .split(|c| c == ' ' || c == '\n' || c == '\r' || c == ',')
        .filter_map(|n| n.parse::<usize>().ok())
        .collect::<Vec<_>>();
    let depth = input[0];

    let target = Vec3::new(input[1], input[2], 0);

    let padding = 20;

    let mut map: Vec<Vec<Node>> =
        vec![vec![Node::new(); target.x + 1 + padding]; target.y + 1 + padding];

    let limits = Vec2::new(map[0].len(), map.len());

    for y in 0..limits.y {
        for x in 0..limits.x {
            if x == 0 {
                map[y][x].gindex = y * 48271;
            } else if y == 0 {
                map[y][x].gindex = x * 16807;
            } else if x == target.x && y == target.y {
            } else {
                map[y][x].gindex = map[y.wrapping_sub(1)][x].erosion_level
                    * map[y][x.wrapping_sub(1)].erosion_level;
            }
            map[y][x].erosion_level = (map[y][x].gindex + depth) % 20183;
            map[y][x].tile = match map[y][x].erosion_level % 3 {
                0 => Tile::Rocky,
                1 => Tile::Wet,
                2 => Tile::Narrow,
                _ => panic!("Unknown Tile"),
            };
        }
    }

    let mut map: Vec<Vec<Vec<Node>>> = vec![map.clone(); 3];

    for gear in 0..3 {
        for y in 0..map[0].len() {
            for x in 0..map[0][0].len() {
                map[gear][y][x].gear = match gear {
                    0 => Gear::Torch,
                    1 => Gear::Climb,
                    2 => Gear::Neither,
                    _ => panic!("Unknown Gear"),
                }
            }
        }
    }

    let mut to_visit: HashSet<Vec3> = HashSet::new();
    to_visit.insert(Vec3::new(0, 0, 0));
    map[0][0][0].time = 0;

    while !to_visit.is_empty() {
        let mut min_time = 99999999999;
        let mut pos = Vec3::new(0, 0, 0);
        for npos in to_visit.iter() {
            if map[npos.gear][npos.y][npos.x].time < min_time {
                min_time = map[npos.gear][npos.y][npos.x].time;
                pos = npos.clone();
            }
        }

        let current_time = map[pos.gear][pos.y][pos.x].time;

        for npos in adj(&pos, &limits) {
            if !map[npos.gear][npos.y][npos.x].visited {
                let next_time = current_time + get_time(&map, &pos, &npos);
                if map[npos.gear][npos.y][npos.x].time > next_time {
                    map[npos.gear][npos.y][npos.x].time = next_time;
                    to_visit.insert(npos.clone());
                }
            }
        }

        map[pos.gear][pos.y][pos.x].visited = true;
        to_visit.remove(&pos);

        if pos == target {
            break;
        }
    }

    println!("{:?} ", map[target.gear][target.y][target.x].time);
}

fn get_time(map: &Vec<Vec<Vec<Node>>>, from: &Vec3, to: &Vec3) -> usize {
    let fromt = map[from.gear][from.y][from.x].tile.clone();
    let tot = map[to.gear][to.y][to.x].tile.clone();
    let tog = map[to.gear][to.y][to.x].gear.clone();

    if tog == Gear::Torch && tot == Tile::Wet {
        return 9999999999;
    }
    if tog == Gear::Climb && tot == Tile::Narrow {
        return 9999999999;
    }
    if tog == Gear::Neither && tot == Tile::Rocky {
        return 9999999999;
    }

    if from.gear != to.gear {
        return 7;
    }

    if fromt == tot {
        return 1;
    }

    return 1;
}

fn adj(pos: &Vec3, limits: &Vec2) -> Vec<Vec3> {
    let mut out: Vec<Vec3> = vec![];
    for (xp, yp, gear) in [
        (pos.x, pos.y.wrapping_sub(1), pos.gear),
        (pos.x.wrapping_sub(1), pos.y, pos.gear),
        (pos.x + 1, pos.y, pos.gear),
        (pos.x, pos.y + 1, pos.gear),
        (pos.x, pos.y, (pos.gear + 2) % 3),
        (pos.x, pos.y, (pos.gear + 1) % 3),
    ]
        .iter()
    {
        if *xp < limits.x && *yp < limits.y {
            out.push(Vec3::new(*xp, *yp, *gear));
        }
    }
    out
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Vec3 {
    x: usize,
    y: usize,
    gear: usize,
}

impl Vec3 {
    fn new(x: usize, y: usize, gear: usize) -> Self {
        Vec3 {
            gear: gear,
            x: x,
            y: y,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum Tile {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
enum Gear {
    Torch,
    Climb,
    Neither,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Node {
    gindex: usize,
    erosion_level: usize,
    tile: Tile,
    gear: Gear,
    time: usize,
    visited: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            gindex: 0,
            erosion_level: 0,
            tile: Tile::Rocky,
            gear: Gear::Neither,
            time: 999999999,
            visited: false,
        }
    }
}
