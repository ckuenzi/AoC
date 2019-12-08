mod intscript;
use itertools::Itertools;
use std::cmp;

fn main() {
    let input = intscript::read_intscript_from_file(include_str!("inputs\\day07a.txt"));
    let mut computers = vec![intscript::Computer::new(input.clone()); 5];

    let mut part1 = 0;
    for phases in (0..5).permutations(5) {
        let mut next_input = 0;
        for (i, computer) in computers.iter_mut().enumerate() {
            computer.add_input(phases[i]);
            next_input = computer.siso(next_input);
            computer.reset();
        }
        part1 = cmp::max(part1, next_input);
    }

    let mut part2 = 0;
    for phases in (5..10).permutations(5) {
        let mut next_input = 0;
        for (i, computer) in computers.iter_mut().enumerate() {
            computer.add_input(phases[i]);
        }
        while !computers[4].halted {
            for computer in computers.iter_mut() {
                next_input = computer.siso(next_input);
                part2 = cmp::max(part2, next_input);
            }
        }
        for computer in computers.iter_mut() {
            computer.reset();
        }
    }
    println!("{}\n{}", part1, part2);
}
