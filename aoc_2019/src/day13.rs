use super::intscript::Computer;

#[aoc(day13, part1)]
fn part1(input: &str) -> usize {
    Computer::new_from_text(input)
        .run()
        .get_outputs()
        .iter()
        .skip(2)
        .step_by(3)
        .filter(|&&t| t == 2)
        .count()
}

#[aoc(day13, part2)]
fn part2(input: &str) -> i64 {
    let mut computer = Computer::new_from_text(input);
    computer.write(0, 2);
    let mut score = 0;
    while !computer.halted {
        let (ball, paddle, new_score) = parse_game_state(computer.run().dump_outputs());
        computer.add_input((ball - paddle).signum());
        score = new_score;
    }
    score
}

fn parse_game_state(outputs: Vec<i64>) -> (i64, i64, i64) {
    let (mut ball, mut paddle, mut score) = (0, 0, 0);
    for chunk in outputs.chunks(3) {
        if chunk[0] == -1 || chunk[1] == 0 {
            score = chunk[2];
        }
        match chunk[2] {
            3 => paddle = chunk[0],
            4 => ball = chunk[0],
            _ => (),
        }
    }
    (ball, paddle, score)
}
