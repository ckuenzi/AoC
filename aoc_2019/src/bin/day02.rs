mod intscript;

fn main() {
    let mut computer = intscript::Computer::new_from_text(include_str!("inputs\\day02a.txt"));

    println!(
        "{}",
        computer
            .write_memory(1, 12)
            .write_memory(2, 2)
            .run()
            .read_memory(0)
    );

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            computer
                .reset()
                .write_memory(1, noun)
                .write_memory(2, verb)
                .run();
            if computer.read_memory(0) == 19690720 {
                println!("{}", 100 * noun + verb);
                break 'outer;
            }
        }
    }
}
