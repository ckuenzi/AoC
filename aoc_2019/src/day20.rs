use hashbrown::HashMap;
use itertools::Itertools;

#[aoc_generator(day20)]
fn maze_gen(input: &str) -> HashMap<Pos, Node> {
    let mut map = HashMap::new();
    let lines = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut portals = HashMap::<String, Vec<Pos>>::new();
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;
    for y in 0_i32..height {
        for x in 0_i32..width {
            let pos = Pos::new(x, y);
            let tile = lines[y as usize][x as usize];
            if tile == ' ' || tile == '#' {
                continue;
            }
            map.insert(pos.clone(), Node::new(lines[y as usize][x as usize]));
            for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
                let pn = Pos::new(x + dir.0, y + dir.1);
                if pn.x >= 0
                    && pn.y >= 0
                    && pn.x < width
                    && pn.y < height
                    && lines[pn.y as usize][pn.x as usize] != '#'
                    && lines[pn.y as usize][pn.x as usize] != ' '
                {
                    map.get_mut(&pos).unwrap().adj.push(pn);
                }
            }
            if tile.is_uppercase() && map.get(&pos).unwrap().adj.len() == 2 {
                let second_letter;
                for adj in map.get(&pos).unwrap().adj.clone() {
                    if lines[adj.y as usize][adj.x as usize].is_uppercase() {
                        second_letter = lines[adj.y as usize][adj.x as usize];
                        let portal;
                        if pos.x == adj.x {
                            if pos.y < adj.y {
                                portal = format!("{}{}", tile, second_letter);
                            } else {
                                portal = format!("{}{}", second_letter, tile);
                            }
                        } else {
                            if pos.x < adj.x {
                                portal = format!("{}{}", tile, second_letter);
                            } else {
                                portal = format!("{}{}", second_letter, tile);
                            }
                        }
                        portals.insert(portal.to_string(), vec![]);
                        map.get_mut(&pos).unwrap().portal = portal;
                        break;
                    }
                }
            }
        }
    }

    for pos in map.keys().clone() {
        if map.get(&pos).unwrap().tile != '.' {
            continue;
        }
        for adj in map.get(&pos).unwrap().adj.clone() {
            if map.get(&adj).unwrap().portal.len() == 2 {
                portals
                    .get_mut(&map.get(&adj).unwrap().portal)
                    .unwrap()
                    .push(pos.clone());
            }
        }
    }
    for (name, sides) in portals {
        if name == "AA" {
            map.get_mut(&sides[0]).unwrap().start = true;
            continue;
        }
        if name == "ZZ" {
            map.get_mut(&sides[0]).unwrap().end = true;
            continue;
        }
        map.get_mut(&sides[0]).unwrap().adj.push(sides[1].clone());
        map.get_mut(&sides[1]).unwrap().adj.push(sides[0].clone());
        map.get_mut(&sides[0]).unwrap().portal = name.to_string();
        map.get_mut(&sides[1]).unwrap().portal = name.to_string();
    }

    map
}

#[aoc(day20, part1)]
fn part1(map: &HashMap<Pos, Node>) -> u32 {
    let mut map = map.clone();
    let start = map.iter().filter(|(_, v)| v.start).next().unwrap().0;

    let mut to_visit = vec![*start];
    let result = loop {
        let pos = to_visit.remove(0);

        if map.get(&pos).unwrap().end {
            println!("found end at distance {}", map.get(&pos).unwrap().distance);
            break map.get(&pos).unwrap().distance;
        }

        for next in map.get(&pos).unwrap().adj.clone() {
            if !map.get(&next).unwrap().visited {
                map.get_mut(&next).unwrap().visited = true;
                map.get_mut(&next).unwrap().distance = map.get(&pos).unwrap().distance + 1;
                to_visit.push(next);
            }
        }
    };
    result
}

#[derive(Debug, Clone)]
struct Node {
    tile: char,
    visited: bool,
    distance: u32,
    portal: String,
    adj: Vec<Pos>,
    start: bool,
    end: bool,
}

impl Node {
    fn new(tile: char) -> Node {
        Node {
            tile,
            visited: false,
            adj: vec![],
            portal: "".to_string(),
            distance: 0,
            start: false,
            end: false,
        }
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
