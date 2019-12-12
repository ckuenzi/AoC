use itertools::Itertools;
use num::integer::lcm;

#[aoc_generator(day12)]
fn system_gen(input: &str) -> Vec<Moon> {
    input
        .lines()
        .map(|l| {
            l.split(|c| c == '=' || c == '>' || c == ',')
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .map(|m| Moon::new(m))
        .collect()
}

#[aoc(day12, part1)]
fn part1(moons: &Vec<Moon>) -> i32 {
    let mut moons = moons.to_vec();
    for _ in 0..1000 {
        step(&mut moons);
    }
    moons.iter().map(|m| m.energy()).sum::<i32>()
}

#[aoc(day12, part2)]
fn part2(moons: &Vec<Moon>) -> u64 {
    let dim = moons[0].pos.len();
    let mut moons = moons.to_vec();
    let start_state = moons.clone();
    let mut periods = vec![0; dim];
    let mut counter = 0;
    while periods.contains(&0) {
        counter += 1;
        step(&mut moons);

        for d in 0..dim {
            if periods[d] != 0 {
                continue;
            }
            let mut eq = true;
            for m in 0..moons.len() {
                if moons[m].pos[d] != start_state[m].pos[d]
                    || moons[m].vel[d] != start_state[m].vel[d]
                {
                    eq = false;
                    break;
                }
            }
            if eq {
                periods[d] = counter;
            }
        }
    }
    periods.iter().fold(1, |acc, p| lcm(acc, *p))
}

fn step(moons: &mut Vec<Moon>) {
    for i in (0..moons.len()).permutations(2) {
        let m1 = i[0];
        let m2 = i[1];
        for i in 0..moons[m1].vel.len() {
            moons[m1].vel[i] += (moons[m2].pos[i] - moons[m1].pos[i]).signum();
        }
    }
    for moon in moons.iter_mut() {
        moon.pos
            .iter_mut()
            .zip(moon.vel.iter())
            .for_each(|(p, v)| *p += v);
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Moon {
    pos: Vec<i32>,
    vel: Vec<i32>,
}

impl Moon {
    pub fn new(pos: Vec<i32>) -> Moon {
        Moon {
            vel: vec![0; pos.len()],
            pos,
        }
    }

    pub fn energy(&self) -> i32 {
        self.pos.iter().map(|c| c.abs()).sum::<i32>()
            * self.vel.iter().map(|c| c.abs()).sum::<i32>()
    }
}

#[test]
fn test_part2() {
    assert_eq!(
        part2(&system_gen(
            "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>",
        )),
        2772
    );
}
