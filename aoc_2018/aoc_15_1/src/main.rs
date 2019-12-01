use std::collections::VecDeque;

fn main() {
    let mut fighters: Vec<Fighter> = vec![];
    let mut fcnt = 0;
    let mut xcnt = 0;
    let mut ycnt = 0;
    let mut map = include_str!("input.txt")
        .lines()
        .fold(vec![], |mut out, line| {
            xcnt = 0;
            out.push(line.chars().fold(vec![], |mut out, c| {
                out.push(match c {
                    '.' => Node::new(Tile::Free, None),
                    '#' => Node::new(Tile::Wall, None),
                    'G' => {
                        fighters.push(Fighter::new(false, xcnt, ycnt, fcnt));
                        fcnt += 1;
                        Node::new(Tile::Fighter, Some(fcnt - 1))
                    }
                    'E' => {
                        fighters.push(Fighter::new(true, xcnt, ycnt, fcnt));
                        fcnt += 1;
                        Node::new(Tile::Fighter, Some(fcnt - 1))
                    }
                    _ => panic!("Unknown input"),
                });
                xcnt += 1;
                out
            }));
            ycnt += 1;
            out
        });

    let mut rounds = 0;
    loop {
        let mut finished = true;
        let mut ids = fighters.clone();
        ids.sort_by(|a, b| {
            if a.pos.y != b.pos.y {
                a.pos.y.cmp(&b.pos.y)
            } else {
                a.pos.x.cmp(&b.pos.x)
            }
        });

        'floop: for i in ids.iter().map(|i| i.id) {
            if fighters[i].hp <= 0 {
                continue;
            }

            let mut targets: Vec<(usize, i32, usize, usize)> = vec![]; //id // hp // //x //y
            for p in adj(&fighters[i].pos).iter() {
                if let Some(id) = map[p.y][p.x].fighter_id {
                    if fighters[i].is_elve != fighters[id].is_elve {
                        targets.push((id, fighters[id].hp, p.x, p.y));
                    }
                }
            }

            if targets.is_empty() {
                let moves = get_move(&map, &fighters, i);
                if !moves.is_empty() {
                    let mv = moves[0].1.clone();
                    map[fighters[i].pos.y][fighters[i].pos.x].tile = Tile::Free;
                    map[fighters[i].pos.y][fighters[i].pos.x].fighter_id = None;
                    map[mv.y][mv.x].fighter_id = Some(i);
                    map[mv.y][mv.x].tile = Tile::Fighter;
                    fighters[i].pos = mv;
                    finished = false;
                }
            }

            for p in adj(&fighters[i].pos).iter() {
                if let Some(id) = map[p.y][p.x].fighter_id {
                    if fighters[i].is_elve != fighters[id].is_elve {
                        targets.push((id, fighters[id].hp, p.x, p.y));
                    }
                }
            }

            if !targets.is_empty() {
                finished = false;
                targets.sort_by(|a, b| {
                    if a.1 != b.1 {
                        a.1.cmp(&b.1)
                    } else {
                        if a.3 != b.3 {
                            a.3.cmp(&b.3)
                        } else {
                            a.2.cmp(&b.2)
                        }
                    }
                });

                fighters[targets[0].0].hp -= fighters[i].attack_power;
                if fighters[targets[0].0].hp <= 0 {
                    map[targets[0].3][targets[0].2].tile = Tile::Free;
                    map[targets[0].3][targets[0].2].fighter_id = None;
                }
            }
        }
        if finished {
            break;
        }
        rounds += 1;
    }
    rounds -= 1;

    let mut result = 0;
    for fighter in fighters {
        if fighter.hp > 0 {
            result += fighter.hp;
        }
    }
    println!("Result: {} * {} = {}", result, rounds, result * rounds);
}

fn adj(pos: &Vec2) -> Vec<Vec2> {
    vec![
        Vec2::new(pos.x, pos.y - 1),
        Vec2::new(pos.x - 1, pos.y),
        Vec2::new(pos.x + 1, pos.y),
        Vec2::new(pos.x, pos.y + 1),
    ]
}

fn get_move(map: &Vec<Vec<Node>>, fighters: &Vec<Fighter>, fid: usize) -> Vec<(usize, Vec2)> {
    let mut bfs = vec![
        vec![
            BfsNode {
                visited: false,
                adjacent: false,
                path: vec![],
                distance: 0
            };
            map[0].len()
        ];
        map.len()
    ];

    let fighter = &fighters[fid];
    let mut queue: VecDeque<Vec2> = VecDeque::new();
    queue.push_back(fighter.pos.clone());
    for i in 0..fighters.len() {
        if fighters[i].hp <= 0 {
            continue;
        }
        if fighters[i].is_elve != fighter.is_elve {
            for (xp, yp) in [
                (fighters[i].pos.x, fighters[i].pos.y - 1),
                (fighters[i].pos.x - 1, fighters[i].pos.y),
                (fighters[i].pos.x + 1, fighters[i].pos.y),
                (fighters[i].pos.x, fighters[i].pos.y + 1),
            ]
                .iter()
            {
                if map[*yp][*xp].tile == Tile::Free {
                    bfs[*yp][*xp].adjacent = true;
                }
            }
        }
    }

    let mut results: Vec<(usize, Vec2)> = vec![];
    //print_bfs(&bfs);
    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        bfs[pos.y][pos.x].path.push(pos.clone());

        if bfs[pos.y][pos.x].adjacent {
            results.push((
                bfs[pos.y][pos.x].distance,
                bfs[pos.y][pos.x].path[1].clone(),
            ));
            continue;
        }

        for (xp, yp) in [
            (pos.x, pos.y - 1),
            (pos.x - 1, pos.y),
            (pos.x + 1, pos.y),
            (pos.x, pos.y + 1),
        ]
            .iter()
        {
            if map[*yp][*xp].tile == Tile::Free && !bfs[*yp][*xp].visited {
                bfs[*yp][*xp].visited = true;
                bfs[*yp][*xp].distance = bfs[pos.y][pos.x].distance + 1;
                bfs[*yp][*xp].path = bfs[pos.y][pos.x].path.clone();
                queue.push_back(Vec2::new(*xp, *yp));
            }
        }
    }

    results.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            if a.1.y != b.1.y {
                a.1.y.cmp(&b.1.y)
            } else {
                a.1.x.cmp(&b.1.x)
            }
        }
    });
    results
}

#[allow(dead_code)]
fn print_bfs(map: &Vec<Vec<BfsNode>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!(
                "{}",
                match map[y][x].adjacent {
                    true => '?',
                    false => '.',
                }
            );
        }
        println!();
    }
}

fn print_map(map: &Vec<Vec<Node>>, fighters: &Vec<Fighter>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!(
                "{}",
                match map[y][x].tile {
                    Tile::Free => '.',
                    Tile::Wall => '#',
                    Tile::Fighter => match fighters[map[y][x].fighter_id.unwrap()].is_elve {
                        true => 'E',
                        false => 'G',
                    },
                }
            );
        }
        println!();
    }
}

#[derive(Clone)]
struct BfsNode {
    visited: bool,
    path: Vec<Vec2>,
    distance: usize,
    adjacent: bool,
}

#[derive(Debug, Clone)]
struct Vec2 {
    x: usize,
    y: usize,
}

impl Vec2 {
    fn new(x: usize, y: usize) -> Self {
        Vec2 { x: x, y: y }
    }
}

#[derive(Debug)]
struct Node {
    tile: Tile,
    fighter_id: Option<usize>,
}

impl Node {
    fn new(t: Tile, fid: Option<usize>) -> Self {
        Node {
            tile: t,
            fighter_id: fid,
        }
    }
}

#[derive(Debug, Clone)]
struct Fighter {
    hp: i32,
    attack_power: i32,
    is_elve: bool,
    pos: Vec2,
    id: usize,
}

impl Fighter {
    fn new(is_elve: bool, posx: usize, posy: usize, id: usize) -> Self {
        Fighter {
            hp: 200,
            attack_power: 3,
            is_elve: is_elve,
            pos: Vec2::new(posx, posy),
            id: id,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Free,
    Wall,
    Fighter,
}
