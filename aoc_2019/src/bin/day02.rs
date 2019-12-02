fn main() {
    task1();
    task2();
}

fn get_input() -> Vec<usize> {
    include_str!("inputs\\day02a.txt")
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

fn task1() {
    let mut memory: Vec<usize> = get_input();
    memory[1] = 12;
    memory[2] = 2;
    run_program(&mut memory);
    println!("{}", memory[0])
}

fn task2() {
    let memory = get_input();
    'outer: for noun in 0..100 {
        for verb in 0..100 {
            let mut modified_memory = memory.clone();
            modified_memory[1] = noun;
            modified_memory[2] = verb;
            run_program(&mut modified_memory);
            if modified_memory[0] == 19690720 {
                println!("{}", 100 * noun + verb);
                break 'outer;
            }
        }
    }
}

fn run_program(memory: &mut Vec<usize>) {
    let mut IP = 0;
    loop {
        let instruction = memory[IP];

        match instruction {
            1 => {
                let target = memory[IP + 3];
                memory[target] = memory[memory[IP + 1]] + memory[memory[IP + 2]];
            }
            2 => {
                let target = memory[IP + 3];
                memory[target] = memory[memory[IP + 1]] * memory[memory[IP + 2]];
            }
            99 => break,
            _ => panic!("Unknown opcode {} at position {}", memory[IP], IP),
        }
        IP += 4;
    }
}

#[test]
fn test_runner() {
    let mut prog_1 = vec![1, 0, 0, 0, 99];
    run_program(&mut prog_1);
    assert_eq!(prog_1, vec!(2, 0, 0, 0, 99));

    let mut prog_2 = vec![2, 3, 0, 3, 99];
    run_program(&mut prog_2);
    assert_eq!(prog_2, vec!(2, 3, 0, 6, 99));

    let mut prog_3 = vec![2, 4, 4, 5, 99, 0];
    run_program(&mut prog_3);
    assert_eq!(prog_3, vec!(2, 4, 4, 5, 99, 9801));

    let mut prog_4 = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    run_program(&mut prog_4);
    assert_eq!(prog_4, vec!(30, 1, 1, 4, 2, 5, 6, 0, 99));
}
