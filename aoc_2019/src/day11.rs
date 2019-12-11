use super::intscript::Computer;
use std::cmp::{max, min};
use std::collections::HashMap;

#[aoc(day11, part1)]
fn part1(input: &str) -> usize {
    run_robot(&mut Computer::new_from_text(input), 0).len()
}

#[aoc(day11, part2)]
fn part2(input: &str) -> String {
    let map = run_robot(&mut Computer::new_from_text(input), 1);
    let mut xl = (std::i64::MAX, std::i64::MIN);
    let mut yl = (std::i64::MAX, std::i64::MIN);
    for (pos, _) in map.iter() {
        xl.0 = min(xl.0, pos.0);
        xl.1 = max(xl.1, pos.0);
        yl.0 = min(yl.0, pos.1);
        yl.1 = max(yl.1, pos.1);
    }

    let mut out = String::from("\n");
    for x in xl.0..=xl.1 {
        for y in yl.0..=yl.1 {
            out = format!(
                "{}{}",
                out,
                match map.get(&(x, y)) {
                    Some(1) => '█',
                    _ => ' ',
                }
            );
        }
        out = format!("{}\n", out);
    }
    out
}

fn run_robot(computer: &mut Computer, mut default: i64) -> HashMap<(i64, i64), i64> {
    let mut map = HashMap::<(i64, i64), i64>::new();
    let mut pos = (0, 0);
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut current_dir: usize = 0;

    while !computer.halted {
        computer.add_input(*map.entry(pos).or_insert(default)).run();
        default = 0;
        *map.get_mut(&pos).unwrap() = computer.get_output().unwrap();
        current_dir = ((current_dir as i64 + computer.get_output().unwrap() * -2 + 3) % 4) as usize;
        pos.0 += directions[current_dir].0;
        pos.1 += directions[current_dir].1;
    }
    map
}
