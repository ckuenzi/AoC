mod intscript;
use itertools::Itertools;
use std::cmp;

fn main() {
    let mut computers =
        vec![intscript::Computer::new_from_text(include_str!("inputs\\day07a.txt")); 5];

    let mut part1 = 0;
    for phases in (0..5).permutations(5) {
        let mut carry = 0;
        for (i, computer) in computers.iter_mut().enumerate() {
            carry = computer.reset().add_input(phases[i]).siso(carry);
        }
        part1 = cmp::max(part1, carry);
    }

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
    println!("{}\n{}", part1, part2);
}
