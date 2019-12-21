use super::intscript::Computer;

#[aoc(day21, part1)]
fn part1(input: &str) -> i64 {
    let mut computer = Computer::new_from_text(input);
    computer.input_string("NOT C J\n");
    computer.input_string("AND D J\n");
    computer.input_string("NOT A T\n");
    computer.input_string("OR T J\n");
    computer.input_string("WALK\n");

    *computer.run().get_outputs().iter().last().unwrap()
}

#[aoc(day21, part2)]
fn part2(input: &str) -> i64 {
    let mut computer = Computer::new_from_text(input);
    computer.input_string("NOT C J\n"); //jump to behind gap
    computer.input_string("AND D J\n"); //if there is ground at the landing
    computer.input_string("OR E T\n");
    computer.input_string("OR H T\n");
    computer.input_string("AND T J\n"); //dont jump if forced to jump into gap
    computer.input_string("AND B T\n");
    computer.input_string("OR B T\n");
    computer.input_string("OR E T\n");
    computer.input_string("NOT T T\n");
    computer.input_string("OR T J\n"); //jump again if jumped before gap
    computer.input_string("NOT A T\n");
    computer.input_string("OR T J\n"); //else jump when right before hole
    computer.input_string("RUN\n");

    *computer.run().get_outputs().iter().last().unwrap()
}
