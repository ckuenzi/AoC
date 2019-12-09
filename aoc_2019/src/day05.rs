use super::intscript::Computer;

#[aoc(day5, part2)]
fn part2(input: &str) -> i64 {
    Computer::new_from_text(input).siso(5)
}

#[aoc(day5, part1)]
fn part1(input: &str) -> i64 {
    Computer::new_from_text(input)
        .add_input(1)
        .run()
        .get_outputs()
        .pop_back()
        .unwrap()
}
