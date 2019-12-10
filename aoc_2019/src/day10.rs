use itertools::Itertools;
use std::f64::consts::PI;
use std::ops::Sub;

#[aoc_generator(day10, part1)]
fn gen(input: &str) -> Vec<Coord> {
    let mut asteroids = vec![];

    for (y, row) in input.lines().enumerate() {
        for (x, char) in row.chars().enumerate() {
            if char == '#' {
                asteroids.push(Coord::new(x as i32, y as i32));
            }
        }
    }
    asteroids
}

#[aoc(day10, part1)]
fn part2(map: &Vec<Coord>) -> String {
    let mut max = 0;
    let mut base = Coord::new(0, 0);
    for current in 0..map.len() {
        let origin = map[current];
        let distances = map.iter().map(|c| *c - origin).collect_vec();
        let mut count = 0;
        for other in 0..distances.len() {
            if other == current {
                continue;
            }
            let mut visible = true;
            for i in 0..distances.len() {
                if other == i || current == i {
                    continue;
                }

                if !distances[other].is_visible(&distances[i])
                    && distances[i].is_visible(&distances[other])
                    && distances[i].x.signum() == distances[other].x.signum()
                    && distances[i].y.signum() == distances[other].y.signum()
                {
                    visible = false;
                    break;
                }
            }
            if visible {
                count += 1;
            }
        }
        if count > max {
            max = count;
            base = map[current];
        }
    }
    let part1 = max;

    let mut asteroids = vec![];
    for coord in map {
        if *coord == base {
            continue;
        }
        let transformed = *coord - base;
        let mut angle = f64::from(transformed.y).atan2(f64::from(transformed.x)) + PI / 2.0;
        if angle < 0.0 {
            angle += 2.0 * PI;
        }
        let distance = transformed.x.pow(2) + transformed.y.pow(2);
        asteroids.push(Asteroid {
            cartesian: transformed,
            original_pos: *coord,
            angle: angle,
            distance: distance,
            destroyed: false,
        });
    }

    let mut asteroids = asteroids
        .iter_mut()
        .sorted_by(|a, b| {
            if a.angle == b.angle {
                a.distance.partial_cmp(&b.distance).unwrap()
            } else {
                a.angle.partial_cmp(&b.angle).unwrap()
            }
        })
        .collect_vec();

    let part2;
    let mut count = 0;
    let mut prev_angle = -1.0;
    'outer: loop {
        for asteroid in &mut asteroids {
            if asteroid.destroyed {
                continue;
            }
            if asteroid.angle == prev_angle {
                continue;
            }
            count += 1;
            prev_angle = asteroid.angle;
            asteroid.destroyed = true;
            if count == 200 {
                part2 = asteroid.original_pos.x * 100 + asteroid.original_pos.y;
                break 'outer;
            }
        }
    }

    format!("{}\n{}", part1, part2).to_string()
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Coord {
    x: i32,
    y: i32,
}
#[derive(Debug, Clone, Copy)]
struct Asteroid {
    original_pos: Coord,
    cartesian: Coord,
    distance: i32,
    angle: f64,
    destroyed: bool,
}
impl Coord {
    pub fn new(x: i32, y: i32) -> Coord {
        Coord { x: x, y: y }
    }

    pub fn is_visible(&self, other: &Coord) -> bool {
        (self.y - other.y) * self.x == self.y * (self.x - other.x)
            && self.x * self.x + self.y * self.y < other.x * other.x + other.y * other.y
    }
}

impl Sub for Coord {
    type Output = Coord;
    fn sub(self, other: Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[test]
fn test_visible_check() {
    assert_eq!(Coord::new(2, 2).is_visible(&Coord::new(1, 0)), false);
    assert_eq!(Coord::new(1, 1).is_visible(&Coord::new(1, 2)), false);
    assert_eq!(Coord::new(-1, 0).is_visible(&Coord::new(2, 1)), false);
    assert_eq!(Coord::new(-1, 0).is_visible(&Coord::new(2, 0)), true);
    assert_eq!(Coord::new(1, 1).is_visible(&Coord::new(2, 2)), true);
    assert_eq!(Coord::new(2, 0).is_visible(&Coord::new(-1, 0)), false);
}
