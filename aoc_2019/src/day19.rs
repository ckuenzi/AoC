use super::intscript::Computer;
use itertools::Itertools;

#[aoc(day19, part1)]
fn part1(input: &str) -> i64 {
    let mut computer = Computer::new_from_text(input);
    let mut sum = 0;
    for (x, y) in (0..50).into_iter().cartesian_product((0..50).into_iter()) {
        sum += check_beam(x, y, &mut computer) as i64;
    }
    sum
}

#[aoc(day19, part2)]
fn part2(input: &str) -> i64 {
    let mut computer = Computer::new_from_text(input);
    let (mut x, mut y) = (3, 3);
    while !(y > 100 && check_beam(x + 99, y - 99, &mut computer)) {
        y += 1;
        while {
            x += 1;
            !check_beam(x, y, &mut computer)
        } {}
    }

    x * 10000 + (y - 99)
}

fn check_beam(x: i64, y: i64, computer: &mut Computer) -> bool {
    computer
        .reset()
        .add_inputs(vec![x, y])
        .run()
        .get_output()
        .unwrap()
        == 1
}
