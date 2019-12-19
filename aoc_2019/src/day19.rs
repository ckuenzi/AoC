use super::intscript::Computer;
use hashbrown::HashMap;

#[aoc(day19, part1)]
fn part1(input: &str) -> i64 {
    let mut computer = Computer::new_from_text(input);
    let mut sum = 0;
    for y in 0..50 {
        for x in 0..50 {
            sum += check_beam(x, y, &mut computer);
        }
    }
    sum
}

#[aoc(day19, part2)]
fn part2(input: &str) -> i64 {
    let mut computer = Computer::new_from_text(input);
    let (mut left, mut right) = (3, 3);
    let mut limits = HashMap::<i64, (i64, i64)>::new(); //line -> left, right;
    let mut line = 3;
    loop {
        for l in (left + 1)..(left + 3) {
            if check_beam(l, line, &mut computer) == 1 {
                left = l;
                break;
            }
        }
        for r in ((right)..(right + 3)).rev() {
            if check_beam(r, line, &mut computer) == 1 {
                right = r;
                break;
            }
        }
        limits.insert(line, (left, right));
        if square_fits((left, line), 100, &limits) {
            break;
        }
        line += 1;
    }

    left * 10000 + (line - 99)
}

fn square_fits(bottom_left: (i64, i64), size: i64, limits: &HashMap<i64, (i64, i64)>) -> bool {
    let bot_line = bottom_left.1;
    let top_line = bot_line - size + 1;
    let left = bottom_left.0;
    let right = left + size - 1;

    limits.contains_key(&top_line)
        && limits.get(&top_line).unwrap().0 <= left
        && limits.get(&bot_line).unwrap().0 <= left
        && limits.get(&top_line).unwrap().1 >= right
        && limits.get(&bot_line).unwrap().1 >= right
}

fn check_beam(x: i64, y: i64, computer: &mut Computer) -> i64 {
    computer
        .reset()
        .add_inputs(vec![x, y])
        .run()
        .get_output()
        .unwrap()
}

#[test]
fn test_square_fit() {
    let mut limits = HashMap::<i64, (i64, i64)>::new();
    limits.insert(1, (1, 1));
    assert_eq!(square_fits((1, 1), 1, &limits), true);
    assert_eq!(square_fits((1, 1), 2, &limits), false);
    limits.insert(2, (1, 2));
    assert_eq!(square_fits((1, 2), 2, &limits), false);
    limits.insert(3, (1, 2));
    assert_eq!(square_fits((1, 3), 2, &limits), true);
    assert_eq!(square_fits((2, 3), 2, &limits), false);
    assert_eq!(square_fits((0, 3), 2, &limits), false);
}
