use super::intscript::Computer;

#[aoc(day9, part1)]
fn part1(input: &str) -> i64{
    Computer::new_from_text(input).siso(1)
}

#[aoc(day9, part2)]
fn part2(input: &str) -> i64{
    Computer::new_from_text(input).siso(2)
}
