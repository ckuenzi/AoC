mod intscript;

fn main() {
    let mut computer = intscript::Computer::new_from_text(include_str!("inputs\\day02a.txt"));

    println!(
        "{}",
        computer
            .write(1, 12)
            .write(2, 2)
            .run()
            .read(0)
    );

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            computer
                .reset()
                .write(1, noun)
                .write(2, verb)
                .run();
            if computer.read(0) == 19690720 {
                println!("{}", 100 * noun + verb);
                break 'outer;
            }
        }
    }
}
