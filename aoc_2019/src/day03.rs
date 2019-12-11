use std::cmp;
use std::collections::HashMap;

#[aoc(day3, part1, part2)]
fn part12(input: &str) -> String {
    let lines = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|segment| {
                    let direction = segment.chars().nth(0).unwrap();
                    let distance = &segment[1..].parse::<i32>().unwrap();
                    match direction {
                        'U' => (0, *distance),
                        'D' => (0, -*distance),
                        'R' => (*distance, 0),
                        'L' => (-*distance, 0),
                        _ => panic!("Unknown direction"),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut field: HashMap<Coord, Node> = HashMap::new();
    let mut right = true;
    let mut left = false;
    for line in lines {
        right = !right;
        left = !left;
        let mut steps = 0;
        let mut pos = Coord { x: 0, y: 0 };
        for mut direction in line {
            while direction != (0, 0) {
                steps += 1;
                let x = direction.0.signum();
                let y = direction.1.signum();
                pos.x += x;
                pos.y += y;
                direction.0 -= x;
                direction.1 -= y;
                let occupied = field.entry(pos).or_insert(Node {
                    pos: pos,
                    left: left,
                    right: right,
                    lsteps: if left { steps } else { std::u32::MAX },
                    rsteps: if left { std::u32::MAX } else { steps },
                });
                if left {
                    occupied.left = true;
                    occupied.lsteps = cmp::min(occupied.lsteps, steps);
                }
                if right {
                    occupied.right = true;
                    occupied.rsteps = cmp::min(steps, occupied.rsteps);
                }
            }
        }
    }
    let closest = field
        .iter()
        .filter(|&a| a.1.left && a.1.right)
        .min_by(|a, b| a.1.pos.norm().cmp(&(b.1.pos.norm())))
        .map(|a| a.1.pos.x.abs() + a.1.pos.y.abs())
        .unwrap();
    let shortest = field
        .iter()
        .filter(|&a| a.1.left && a.1.right)
        .min_by(|a, b| (a.1.rsteps + a.1.lsteps).cmp(&(b.1.rsteps + b.1.lsteps)))
        .map(|a| a.1.rsteps + a.1.lsteps)
        .unwrap();
    format!("{}\n{}", closest, shortest)
}

#[derive(Clone, Hash, Eq, PartialEq, Debug, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Node {
    pos: Coord,
    left: bool,
    right: bool,
    lsteps: u32,
    rsteps: u32,
}

impl Coord {
    fn norm(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}
