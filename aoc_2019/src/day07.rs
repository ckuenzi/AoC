use super::intscript::Computer;
use itertools::Itertools;
use std::cmp;

#[aoc(day7, part1)]
fn part1(input: &str) -> i64 {
    let mut computers = vec![Computer::new_from_text(input); 5];
    let mut res = 0;
    for phases in (0..5).permutations(5) {
        let mut carry = 0;
        for (i, computer) in computers.iter_mut().enumerate() {
            carry = computer.reset().add_input(phases[i]).siso(carry);
        }
        res = cmp::max(res, carry)
    }
    res
}

#[aoc(day7, part2)]
fn part2(input: &str) -> i64 {
    let mut computers = vec![Computer::new_from_text(input); 5];
    let mut part2 = 0;
    for phases in (5..10).permutations(5) {
        let mut carry = 0;
        for (i, computer) in computers.iter_mut().enumerate() {
            computer.add_input(phases[i]);
        }
        while !computers[4].halted {
            for computer in computers.iter_mut() {
                carry = computer.siso(carry);
                part2 = cmp::max(part2, carry);
            }
        }
        for computer in computers.iter_mut() {
            computer.reset();
        }
    }
    part2
}
