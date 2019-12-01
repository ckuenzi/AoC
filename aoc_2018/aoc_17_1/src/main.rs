use std::cmp;

fn main() {
    let mut map = vec![vec![Node::new(); 600]; 2000];
    map[0][500].tile = Tile::Source;
    let input = include_str!("input.txt").lines();

    for line in input {
        let parts: Vec<_> = line
            .split(|c| c == '=' || c == ',' || c == ' ' || c == '.')
            .collect();
        let (xs, xe, ys, ye): (usize, usize, usize, usize);
        if parts[0].eq("x") {
            xs = parts[1].parse::<usize>().unwrap();
            xe = xs;
            ys = parts[4].parse::<usize>().unwrap();
            ye = parts[6].parse::<usize>().unwrap();
        } else {
            ys = parts[1].parse::<usize>().unwrap();
            ye = ys;
            xs = parts[4].parse::<usize>().unwrap();
            xe = parts[6].parse::<usize>().unwrap();
        }
        for y in ys..=ye {
            for x in xs..=xe {
                map[y][x].tile = Tile::Clay;
            }
        }
    }

    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_y = map.len();
    let mut min_x = map[0].len();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x].tile == Tile::Clay {
                max_x = cmp::max(max_x, x);
                max_y = cmp::max(max_y, y);
                min_x = cmp::min(min_x, x);
                min_y = cmp::min(min_y, y);
            }
        }
    }
    println!("x: {}..{} \ny: {}..{}", min_x, max_x, min_y, max_y);

    for i in 0..10000 {
        if i % 1 == 0 {
            map[1][500].tile = Tile::Falling;
        }

        for y in 0..max_y {
            for x in min_x..max_x {
                match map[y][x].tile {
                    Tile::Falling => {
                        if map[y + 1][x].tile == Tile::Falling {
                            continue;
                        }
                        if map[y + 1][x].tile == Tile::Free {
                            map[y + 1][x].tile = Tile::Falling;
                        } else {
                            if map[y][x + 1].tile == Tile::Free {
                                map[y][x + 1].tile = Tile::Falling;
                            }
                            if map[y][x - 1].tile == Tile::Free {
                                map[y][x - 1].tile = Tile::Falling;
                            }
                            if map[y][x - 1].tile == Tile::Clay {
                                let mut cnt = x;
                                let mut visited: Vec<usize> = vec![x];
                                loop {
                                    cnt += 1;
                                    match map[y][cnt].tile {
                                        Tile::Falling => {
                                            visited.push(cnt);
                                            continue;
                                        }
                                        Tile::Clay => {
                                            for visit in visited.iter() {
                                                map[y][*visit].tile = Tile::Water;
                                            }
                                        }
                                        _ => break,
                                    }
                                }
                            }
                        }
                    }

                    _ => (),
                }
            }
        }
    }

    let mut total = 0;
    let mut retained = 0;
    for y in min_y..map.len() {
        for x in 0..map[0].len() {
            match map[y][x].tile {
                Tile::Water => {
                    total += 1;
                    retained += 1
                }
                Tile::Falling => total += 1,
                _ => (),
            }
        }
    }
    print_map(&map);
    println!("Total: {}", total + 11);
    println!("Retained: {}", retained);
}

fn print_map(map: &Vec<Vec<Node>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            print!(
                "{}",
                match map[y][x].tile {
                    Tile::Free => '.',
                    Tile::Clay => '#',
                    Tile::Source => '+',
                    Tile::Water => '~',
                    Tile::Falling => '|',
                }
            )
        }
        println!();
    }
}

#[derive(Clone)]
struct Node {
    tile: Tile,
}

impl Node {
    fn new() -> Self {
        Node { tile: Tile::Free }
    }
}

#[derive(Clone, PartialEq)]
enum Tile {
    Clay,
    Free,
    Source,
    Water,
    Falling,
}
