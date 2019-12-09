use super::intscript;

#[aoc(day2, part1)]
fn part1(input: &str) -> i64 {
    let mut computer = intscript::Computer::new_from_text(input);

    computer.write(1, 12).write(2, 2).run().read(0)
}

#[aoc(day2, part2)]
fn part2(input: &str) -> i64 {
    let mut computer = intscript::Computer::new_from_text(input);

    'outer: for noun in 0..100 {
        for verb in 0..100 {
            computer.reset().write(1, noun).write(2, verb).run();
            if computer.read(0) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}
