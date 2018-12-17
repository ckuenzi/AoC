fn main() {
    let opcodes = [
        Opcode::bori,
        Opcode::borr,
        Opcode::addi,
        Opcode::muli,
        Opcode::addr,
        Opcode::bani,
        Opcode::gtri,
        Opcode::setr,
        Opcode::gtrr,
        Opcode::seti,
        Opcode::eqir,
        Opcode::eqrr,
        Opcode::mulr,
        Opcode::eqri,
        Opcode::gtir,
        Opcode::banr,
    ];

    let mut tests: Vec<Vec<Opcode>> = vec![];

    for _ in 0..16 {
        tests.push(opcodes.to_vec());
    }

    let mut input_big = include_str!("input.txt").split("\n\n\n\n");
    let mut input = input_big.next().unwrap().lines();
    let input2 = input_big.next().unwrap().lines();

    let mut result = 0;
    loop {
        let before = parse(input.next().unwrap());
        let instruction = parse(input.next().unwrap());
        let after = parse(input.next().unwrap());

        let mut count = 0;
        let mut possible_opcodes = vec![];
        for opcode in opcodes.to_vec().iter() {
            if execute(*opcode, &instruction, &before) == after {
                count += 1;
                possible_opcodes.push(*opcode);
            }
        }
        if count >= 3 {
            result += 1;
        }

        tests[instruction[0]] = tests[instruction[0]].iter().fold(vec![], |mut out, o| {
            if possible_opcodes.contains(o) {
                out.push(*o);
            }
            out
        });

        if let Some(_) = input.next() {
            continue;
        } else {
            break;
        }
    }

    println!("Number of codes > 3: {}", result);
    println!("{:?}", tests);

    println!(
        "Register values {:?} ",
        input2.fold([0, 0, 0, 0], |reg, line| {
            let instruction = parse(line);
            execute(opcodes[instruction[0]], &instruction, &reg)
        })
    );
}

fn parse(input: &str) -> [usize; 4] {
    let nums = input
        .split(|c| c == '[' || c == ' ' || c == ']' || c == ',')
        .filter_map(|i| i.parse::<usize>().ok())
        .collect::<Vec<_>>();
    [nums[0], nums[1], nums[2], nums[3]]
}

#[allow(non_snake_case)]
fn execute(opcode: Opcode, instruction: &[usize; 4], reg: &[usize; 4]) -> [usize; 4] {
    let A = instruction[1];
    let B = instruction[2];
    let C = instruction[3];
    let mut out: [usize; 4] = reg.clone();
    out[C] = match opcode {
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
    out
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
