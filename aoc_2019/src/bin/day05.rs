mod intscript;

fn main() {
    let mut computer = intscript::Computer::new_from_text(include_str!("inputs\\day05a.txt"));

    println!(
        "{}",
        computer
            .add_input(1)
            .run()
            .get_outputs()
            .pop_back()
            .unwrap()
    );

    println!("{}", computer.reset().siso(5));
}
