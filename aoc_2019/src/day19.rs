use super::intscript::Computer;

#[aoc(day19, part1)]
fn part1(input: &str) -> i64 {
    let mut computer = Computer::new_from_text(input);
    let mut sum = 0;
    for y in 0..50 {
        for x in 0..50 {
            sum += check_beam(x, y, &mut computer) as i64;
        }
    }
    sum
}

#[aoc(day19, part2)]
fn part2(input: &str) -> i64 {
    let mut computer = Computer::new_from_text(input);
    let mut left = 3;
    let mut line = 3;
    loop {
        for l in (left + 1)..(left + 3) {
            if check_beam(l, line, &mut computer) {
                left = l;
                break;
            }
        }
        if line > 100
            && check_beam(left + 99, line, &mut computer)
            && check_beam(left, line - 99, &mut computer)
            && check_beam(left + 99, line - 99, &mut computer)
        {
            break;
        }
        line += 1;
    }

    left * 10000 + (line - 99)
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
