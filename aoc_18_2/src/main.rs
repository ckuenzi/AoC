use std::collections::HashMap;

fn main() {
    let mut map: Vec<Vec<Tile>> = vec![];
    let input = include_str!("input.txt").lines().collect::<Vec<_>>();
    for y in 0..input.len() {
        map.push(vec![]);
        let line = input[y].chars().collect::<Vec<_>>();
        for x in 0..input[y].len() {
            map[y].push(match line[x] {
                '.' => Tile::Free,
                '|' => Tile::Tree,
                '#' => Tile::Lumber,
                _ => panic!("Unknown Symbol"),
            })
        }
    }

    let mut memory: HashMap<Vec<Vec<Tile>>, usize> = HashMap::new();
    let limits = (map[0].len(), map.len());
    let mut i = 0;
    let steps = 1000000000;
    while i < steps {
        i += 1;
        let old = map.clone();
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                let mut cnt_tree = 0;
                let mut cnt_lumber = 0;
                for (xn, yn) in adj(x, y, limits) {
                    match old[yn][xn] {
                        Tile::Tree => cnt_tree += 1,
                        Tile::Lumber => cnt_lumber += 1,
                        _ => (),
                    }
                }
                map[y][x] = match old[y][x] {
                    Tile::Free => {
                        if cnt_tree >= 3 {
                            Tile::Tree
                        } else {
                            Tile::Free
                        }
                    }
                    Tile::Tree => {
                        if cnt_lumber >= 3 {
                            Tile::Lumber
                        } else {
                            Tile::Tree
                        }
                    }
                    Tile::Lumber => {
                        if cnt_tree >= 1 && cnt_lumber >= 1 {
                            Tile::Lumber
                        } else {
                            Tile::Free
                        }
                    }
                }
            }
        }

        if let Some(last_iter) = memory.insert(old, i) {
            let cycle = i - last_iter;
            while i + cycle < steps {
                i += cycle;
            }
        }
        println!("{}", i);
    }

    let mut cnt_tree = 0;
    let mut cnt_lumber = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            match map[y][x] {
                Tile::Tree => cnt_tree += 1,
                Tile::Lumber => cnt_lumber += 1,
                _ => (),
            }
        }
    }
    println!(
        "Result: {} Trees * {} Lumber = {}",
        cnt_tree,
        cnt_lumber,
        cnt_tree * cnt_lumber
    );
}

fn adj(x: usize, y: usize, limits: (usize, usize)) -> Vec<(usize, usize)> {
    let mut out: Vec<(usize, usize)> = vec![];
    for (xp, yp) in [
        (x.wrapping_sub(1), y.wrapping_sub(1)),
        (x, y.wrapping_sub(1)),
        (x + 1, y.wrapping_sub(1)),
        (x.wrapping_sub(1), y),
        (x + 1, y),
        (x.wrapping_sub(1), y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .iter()
    {
        if *xp < limits.0 && *yp < limits.1 {
            out.push((*xp, *yp));
        }
    }
    out
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<Tile>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!(
                "{}",
                match map[y][x] {
                    Tile::Tree => '|',
                    Tile::Free => '.',
                    Tile::Lumber => '#',
                }
            );
        }
        println!();
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum Tile {
    Free,
    Tree,
    Lumber,
}
