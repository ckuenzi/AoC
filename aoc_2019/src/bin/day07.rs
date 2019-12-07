mod intscript;
use itertools::Itertools;
use std::cmp;

fn main() {
    let input = intscript::read_intscript_from_file(include_str!("inputs\\day07a.txt"));
    let mut computers = vec![intscript::Computer::new(input.clone()); 5];

    let mut res = 0;
    for phases in (0..5).permutations(5) {
        let mut next_input = 0;
        for (i, computer) in computers.iter_mut().enumerate() {
            computer.add_input(phases[i]);
            next_input = computer.siso(next_input);
            computer.reset();
        }
        res = cmp::max(res, next_input);
    }
    println!("{}", res);

    //part2
    for phases in (5..10).permutations(5) {
        let mut start_value = 0;
        for (i, computer) in computers.iter_mut().enumerate() {
            computer.add_input(phases[i]);
        }
        while !computers[4].halted {
            for computer in computers.iter_mut() {
                start_value = computer.siso(start_value);
                res = cmp::max(res, start_value);
            }
        }
        for computer in computers.iter_mut() {
            computer.reset();
        }
    }
    println!("{}", res);
}
