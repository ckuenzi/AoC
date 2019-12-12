use itertools::Itertools;
use num::integer::lcm;
use hashbrown::HashMap;

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
    let mut periods = vec![0; dim];
    let mut counter = 0;

    let mut past_states = vec![HashMap::<(Vec<i32>, Vec<i32>), u64>::new(); dim];
    while periods.contains(&0) {
        let (pos, vel) = separate_axes(&mut moons);
        for d in 0..dim {
            if periods[d] != 0 {
                continue;
            }
            if past_states[d].contains_key(&(pos[d].to_vec(), vel[d].to_vec())) {
                periods[d] = counter;
            } else {
                past_states[d].insert((pos[d].to_vec(), vel[d].to_vec()), counter);
            }
        }
        counter += 1;
        step(&mut moons);
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

fn separate_axes(moons: &mut Vec<Moon>) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut pos = vec![vec![0; moons.len()]; moons[0].pos.len()];
    let mut vel = vec![vec![0; moons.len()]; moons[0].pos.len()];

    for i in 0..moons.len() {
        for d in 0..moons[0].pos.len() {
            pos[d][i] = moons[i].pos[d];
            vel[d][i] = moons[i].vel[d];
        }
    }
    (pos, vel)
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
