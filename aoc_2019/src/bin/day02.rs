mod intscript;

fn main() {
    let input = intscript::read_intscript_from_file(include_str!("inputs\\day02a.txt"));
    let mut computer = intscript::Computer::new(input);
    computer.write_memory(1, 12);
    computer.write_memory(2, 2);
    computer.run();
    println!("{}", computer.read_memory(0));

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            computer.reset();
            computer.write_memory(1, noun);
            computer.write_memory(2, verb);
            computer.run();
            if computer.read_memory(0) == 19690720 {
                println!("{}", 100 * noun + verb);
                break 'outer;
            }
        }
    }
}