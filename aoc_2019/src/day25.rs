use super::intscript::Computer;
use std::io;

#[aoc(day25, part1)]
#[allow(unused_must_use)]
fn main(input: &str) -> i32 {
    let mut computer = Computer::new_from_text(input);
    println!("{}", computer.run().output_string());
    let mut input = String::new();
    while !computer.halted {
        io::stdin().read_line(&mut input);
        computer.input_string(&input);
        println!("{}", computer.run().output_string());
    }
    352325632
}
