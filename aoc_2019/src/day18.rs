use hashbrown::HashMap;
use itertools::Itertools;

#[aoc(day18, part1)]
fn part1(input: &str) -> u32 {
    let mut map = construct_map(input);
    let start = *map.iter().find(|a| a.1.tile == '@').unwrap().0;
    let mut kk_paths = HashMap::<char, HashMap<char, Path>>::new();
    let mut keys = vec![('@', start)];
    for (k, v) in map.iter() {
        if v.tile.is_lowercase() {
            keys.push((v.tile, k.clone()));
        }
    }

    for key in keys.iter() {
        kk_paths.insert(key.0, to_key_paths(key.1, &mut map));
    }
    let to_visit = keys.iter().map(|k| k.0).collect_vec();
    let mut known_routes = HashMap::<(char, Vec<char>), u32>::new();

    find_shortest_route(
        '@',
        &kk_paths,
        Vec::<char>::new(),
        to_visit,
        &mut known_routes,
    )
}

#[aoc(day18, part2)]
fn part2(input: &str) -> u32 {
    let mut map = construct_map(input);
    let start = *map.iter().find(|a| a.1.tile == '@').unwrap().0;
    for dir in [(-1, -1), (-1, 1), (1, -1), (1, 1)].iter() {
        let pn = Pos::new(start.x + dir.0, start.y + dir.1);
        map.get_mut(&pn).unwrap().tile = '@'
    }
    for dir in [(-1, 0), (0, 1), (1, 0), (0, -1), (0, 0)].iter() {
        let pn = Pos::new(start.x + dir.0, start.y + dir.1);
        map.get_mut(&pn).unwrap().tile = '#'
    }

    let lines = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;
    let mut sub_maps = Vec::<String>::new();
    for area in [
        (0, width / 2, 0, height / 2),
        (width / 2, width, 0, height / 2),
        (0, width / 2, height / 2, height),
        (width / 2, width, height / 2, height),
    ]
    .iter()
    {
        let mut new_map = String::new();
        for y in area.0..area.1 {
            for x in area.2..area.3 {
                new_map.push(map.get(&Pos::new(x, y)).unwrap().tile);
            }
            new_map.push('\n');
        }
        sub_maps.push(new_map);
    }

    let mut new_maps = vec![];
    for map in sub_maps {
        let keys = map.chars().filter(|c| c.is_lowercase()).collect_vec();
        let mut repl_map = map.clone();
        for c in map.chars() {
            if c.is_uppercase() && keys.iter().find(|&&k| k.to_ascii_uppercase() == c) == None {
                repl_map = repl_map.replace(c, ".");
            }
        }
        new_maps.push(repl_map);
    }

    new_maps.iter().map(|m| part1(m.as_str())).sum::<u32>()
}

fn construct_map(input: &str) -> HashMap<Pos, Node> {
    let lines = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;
    let mut map = HashMap::<Pos, Node>::new();
    for y in 0_i32..height {
        for x in 0_i32..width {
            let pos = Pos::new(x, y);
            map.insert(pos.clone(), Node::new(lines[y as usize][x as usize]));
            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
                let pn = Pos::new(x + dir.0, y + dir.1);
                if pn.x >= 0
                    && pn.y >= 0
                    && pn.x < width
                    && pn.y < height
                    && lines[pn.y as usize][pn.x as usize] != '#'
                {
                    map.get_mut(&pos).unwrap().adj.push(pn);
                }
            }
        }
    }
    map
}

fn find_shortest_route(
    start: char,
    kk_paths: &HashMap<char, HashMap<char, Path>>,
    mut keys: Vec<char>,
    mut to_visit: Vec<char>,
    known_routes: &mut HashMap<(char, Vec<char>), u32>,
) -> u32 {
    to_visit = to_visit
        .iter()
        .filter(|k| **k != start)
        .map(|k| *k)
        .collect_vec();
    keys.push(start);

    if known_routes.get(&(start, to_visit.to_vec())) != None {
        return *known_routes.get(&(start, to_visit)).unwrap();
    }

    if to_visit.is_empty() {
        return 0;
    }

    let visible_keys = get_visible_keys(start, kk_paths, &keys);
    let mut shortest = std::u32::MAX;
    for target_key in to_visit.iter() {
        if visible_keys.iter().find(|&&k| k == *target_key) == None {
            continue;
        }

        let distance = kk_paths
            .get(&start)
            .unwrap()
            .get(&target_key)
            .unwrap()
            .distance;
        let target_shortest = find_shortest_route(
            *target_key,
            kk_paths,
            keys.to_vec(),
            to_visit.to_vec(),
            known_routes,
        );
        let total_distance = target_shortest + distance;
        if total_distance < shortest {
            shortest = total_distance;
        }
    }
    if to_visit.len() < 23 {
        known_routes.insert((start, to_visit), shortest);
    }
    shortest
}

fn get_visible_keys(
    start: char,
    kk_paths: &HashMap<char, HashMap<char, Path>>,
    keys: &Vec<char>,
) -> Vec<char> {
    let mut out = vec![];
    'outer: for (target, path) in kk_paths.get(&start).unwrap() {
        for &key_needed in path.doors.iter() {
            if keys.iter().find(|&&k| k.to_ascii_uppercase() == key_needed) == None {
                continue 'outer;
            }
        }
        out.push(*target);
    }
    out
}

fn to_key_paths(start: Pos, map: &mut HashMap<Pos, Node>) -> HashMap<char, Path> {
    let mut to_visit = vec![start];
    let mut paths = HashMap::<char, Path>::new();
    for node in map.values_mut() {
        node.reset();
    }
    while !to_visit.is_empty() {
        let pos = to_visit.remove(0);
        let current_tile = map.get(&pos).unwrap().tile;
        if current_tile.is_uppercase() {
            map.get_mut(&pos).unwrap().doors.push(current_tile);
        }
        if current_tile.is_lowercase() {
            paths.insert(
                current_tile,
                Path {
                    distance: map.get_mut(&pos).unwrap().distance,
                    doors: map.get_mut(&pos).unwrap().doors.to_vec(),
                },
            );
        }

        for next in map.get(&pos).unwrap().adj.clone() {
            if !map.get(&next).unwrap().visited {
                map.get_mut(&next).unwrap().visited = true;
                map.get_mut(&next).unwrap().distance = map.get(&pos).unwrap().distance + 1;
                let mut doors = map.get(&pos).unwrap().doors.to_vec();
                map.get_mut(&next).unwrap().doors.append(&mut doors);
                to_visit.push(next);
            }
        }
    }
    paths.remove(&map.get(&start).unwrap().tile);
    paths
}

#[derive(Debug, Clone)]
struct Path {
    doors: Vec<char>,
    distance: u32,
}

#[derive(Debug, Clone)]
struct Node {
    tile: char,
    visited: bool,
    distance: u32,
    adj: Vec<Pos>,
    doors: Vec<char>,
}

impl Node {
    fn new(tile: char) -> Node {
        Node {
            tile,
            visited: false,
            adj: vec![],
            distance: 0,
            doors: vec![],
        }
    }

    pub fn reset(&mut self) {
        self.visited = false;
        self.distance = 0;
        self.doors = vec![];
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }
}

#[test]
fn test_part1() {
    assert_eq!(
        part1(
            "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################",
        ),
        136
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(
            "#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######",
        ),
        8
    );
}
