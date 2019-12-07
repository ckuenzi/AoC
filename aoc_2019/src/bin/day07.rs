mod intscript;
use permutohedron::heap_recursive;
use std::cmp;

fn main() {
    let input = intscript::read_intscript_from_file(include_str!("inputs\\day07a.txt"));
    let mut computers = Vec::new();
    for i in [0_i32, 1, 2, 3, 4].iter() {
        computers.push(intscript::Computer::new(input.clone(), *i));
    }

    let mut permutations = Vec::new();
    heap_recursive(&mut [0, 1, 2, 3, 4], |permutation| {
        permutations.push(permutation.to_vec())
    });

    let mut res = 0;
    for phases in permutations.iter() {
        let mut start_value = 0;
        for (i, computer) in computers.iter_mut().enumerate() {
            computer.add_inputs(vec![phases[i], start_value]);
            computer.run_all();
            start_value = computer.get_output().unwrap();
            computer.reset();
        }
        res = cmp::max(res, start_value);
    }
    println!("{}", res);

    //part2
    let mut permutations = Vec::new();
    heap_recursive(&mut [5, 6, 7, 8, 9], |permutation| {
        permutations.push(permutation.to_vec())
    });

    for phases in permutations {
        let mut start_value = 0;
        for (i, computer) in computers.iter_mut().enumerate() {
            computer.add_input(phases[i]);
        }
        loop {
            for computer in computers.iter_mut() {
                computer.add_input(start_value);
                computer.run_till_input();
                match computer.get_output() {
                    Some(out) => start_value = out,
                    None => continue,
                }
                res = cmp::max(res, start_value);
            }
            if computers[4].halted {
                break;
            }
        }
        for computer in computers.iter_mut() {
            computer.reset();
        }
    }
    println!("{}", res);
}
