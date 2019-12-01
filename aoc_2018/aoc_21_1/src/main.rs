fn main() {
    let mut input = include_str!("input.txt").lines();
    let ip_i = input
        .next()
        .unwrap()
        .split(' ')
        .nth(1)
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let instructions = input.map(|line| parse(line)).collect::<Vec<_>>();

    let mut known = vec![false; 20000000];

    let mut reg = [0, 0, 0, 0, 0, 0];
    loop {
        let instruction = instructions[reg[ip_i]];
        execute(instruction.0, &instruction.1, &mut reg);
        if reg[ip_i] == 28 {
            if known[reg[3]] {
                println!("Result: {}", reg[3]);
                break;
            } else {
                known[reg[3]] = true;
            }
        }
        reg[ip_i] += 1;
    }
}

fn parse(input: &str) -> (Opcode, [usize; 3]) {
    let nums = input.split(' ').collect::<Vec<_>>();

    (
        match nums[0] {
            "addi" => Opcode::addi,
            "bori" => Opcode::bori,
            "borr" => Opcode::borr,
            "muli" => Opcode::muli,
            "addr" => Opcode::addr,
            "bani" => Opcode::bani,
            "gtri" => Opcode::gtri,
            "setr" => Opcode::setr,
            "gtrr" => Opcode::gtrr,
            "seti" => Opcode::seti,
            "eqir" => Opcode::eqir,
            "eqrr" => Opcode::eqrr,
            "mulr" => Opcode::mulr,
            "eqri" => Opcode::eqri,
            "gtir" => Opcode::gtir,
            "banr" => Opcode::banr,
            _ => panic!("Unknown Opcode"),
        },
        [
            nums[1].parse::<usize>().unwrap(),
            nums[2].parse::<usize>().unwrap(),
            nums[3].parse::<usize>().unwrap(),
        ],
    )
}

#[allow(non_snake_case)]
fn execute(opcode: Opcode, instruction: &[usize; 3], reg: &mut [usize; 6]) {
    let A = instruction[0];
    let B = instruction[1];
    let C = instruction[2];
    reg[C] = match opcode {
        Opcode::addr => reg[A] + reg[B],
        Opcode::addi => reg[A] + B,
        Opcode::mulr => reg[A] * reg[B],
        Opcode::muli => reg[A] * B,
        Opcode::banr => reg[A] & reg[B],
        Opcode::bani => reg[A] & B,
        Opcode::borr => reg[A] | reg[B],
        Opcode::bori => reg[A] | B,
        Opcode::setr => reg[A],
        Opcode::seti => A,
        Opcode::gtir => match A > reg[B] {
            true => 1,
            false => 0,
        },
        Opcode::gtri => match reg[A] > B {
            true => 1,
            false => 0,
        },
        Opcode::gtrr => match reg[A] > reg[B] {
            true => 1,
            false => 0,
        },
        Opcode::eqir => match A == reg[B] {
            true => 1,
            false => 0,
        },
        Opcode::eqri => match reg[A] == B {
            true => 1,
            false => 0,
        },
        Opcode::eqrr => match reg[A] == reg[B] {
            true => 1,
            false => 0,
        },
    };
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(non_camel_case_types)]
enum Opcode {
    addr,
    addi,
    mulr,
    muli,
    banr,
    bani,
    borr,
    bori,
    setr,
    seti,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,
}
