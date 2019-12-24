use hashbrown::{HashMap, HashSet};

#[aoc(day24, part1)]
fn part1(input: &str) -> u64 {
    let mut eris = grid_gen(input);
    let mut past_states = HashSet::new();
    loop {
        if past_states.contains(&eris) {
            break;
        } else {
            past_states.insert(eris.clone());
        }
        let old = eris.clone();
        for y in 0_usize..5 {
            for x in 0_usize..5 {
                let mut adj = 0;
                for dir in [(-1_i32, 0_i32), (1, 0), (0, -1), (0, 1)].into_iter() {
                    let xn = (x as i32 + dir.0) as usize;
                    let yn = (y as i32 + dir.1) as usize;
                    if yn >= eris.len() || xn >= eris[0].len() {
                        continue;
                    }
                    adj += old[yn][xn];
                }
                if eris[y][x] == 1 && adj != 1 {
                    eris[y][x] = 0;
                } else if eris[y][x] == 0 && (adj == 1 || adj == 2) {
                    eris[y][x] = 1;
                }
            }
        }
    }
    let mut value = 1_u64;
    let mut out = 0_u64;
    for y in 0..5 {
        for x in 0..5 {
            out += (eris[y][x] * value) as u64;
            value *= 2;
        }
    }
    out
}

#[aoc(day24, part2)]
fn part2(input: &str) -> u64 {
    let mut eris = HashMap::<i32, Vec<Vec<u64>>>::new();
    let empty = grid_gen("");
    eris.insert(0, grid_gen(input));
    eris.insert(1, empty.clone());
    eris.insert(-1, empty.clone());
    let mut levels = 0;

    for _ in 0..200 {
        levels += 1;
        eris.insert(levels + 1, empty.clone());
        eris.insert(-levels - 1, empty.clone());
        let old = eris.clone();
        for level in -levels..=levels {
            for y in 0_usize..5 {
                for x in 0_usize..5 {
                    if x == 2 && y == 2 {
                        continue;
                    }
                    let adj = check(x as i32, y as i32, level, &old);
                    if eris.get(&level).unwrap()[y][x] == 1 && adj != 1 {
                        eris.get_mut(&level).unwrap()[y][x] = 0;
                    } else if eris.get(&level).unwrap()[y][x] == 0 && (adj == 1 || adj == 2) {
                        eris.get_mut(&level).unwrap()[y][x] = 1;
                    }
                }
            }
        }

    }


    let mut out = 0;
    for level in -levels..=levels {
        for y in 0..5 {
            for x in 0..5 {
                out += eris.get(&level).unwrap()[y][x];
            }
        }
    }
    out
}

fn check(x: i32, y: i32, z: i32, eris: &HashMap<i32, Vec<Vec<u64>>>) -> u64 {
    let mut out = 0;
    for dir in [(-1_i32, 0_i32), (1, 0), (0, -1), (0, 1)].into_iter() {
        let xn = x + dir.0;
        let yn = y + dir.1;
        if xn == -1 {
            out += eris.get(&(z + 1)).unwrap()[2][1];
        } else if yn == -1 {
            out += eris.get(&(z + 1)).unwrap()[1][2];
        } else if xn == 5 {
            out += eris.get(&(z + 1)).unwrap()[2][3];
        } else if yn == 5 {
            out += eris.get(&(z + 1)).unwrap()[3][2];
        } else if xn == 2 && yn == 2 {
            let r = match (x, y) {
                (1, 2) => (0, 0, 0, 4),
                (3, 2) => (4, 4, 0, 4),
                (2, 1) => (0, 4, 0, 0),
                (2, 3) => (0, 4, 4, 4),
                _ => panic!("u dun goofd"),
            };
            for yy in r.2..=r.3 {
                for xx in r.0..=r.1 {
                    out += eris.get(&(z - 1)).unwrap()[yy as usize][xx as usize];
                }
            }
        } else {
            out += eris.get(&z).unwrap()[yn as usize][xn as usize];
        }
    }
    out
}

fn grid_gen(input: &str) -> Vec<Vec<u64>> {
    let mut out = vec![vec![0_u64; 5]; 5];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            out[y][x] = (c == '#') as u64;
        }
    }
    out
}
