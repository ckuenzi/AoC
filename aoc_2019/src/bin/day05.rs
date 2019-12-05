fn main() {
    task1();
    task2();
}

fn get_input() -> Vec<i32> {
    include_str!("inputs\\day05a.txt")
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn task1() {
    let mut memory: Vec<i32> = get_input();
    let inputs = vec![1];
    run_program(&mut memory, inputs);
}

fn task2() {
    let mut memory: Vec<i32> = get_input();
    memory.append(&mut vec![0, 0, 0]);
    let inputs = vec![5];
    run_program(&mut memory, inputs);
}
#[allow(non_snake_case)]
fn run_program(memory: &mut Vec<i32>, inputs: Vec<i32>) {
    let mut IP: usize = 0;
    let mut inputs = inputs.into_iter();

    loop {
        let mut p1 = IP + 1;
        let mut p2 = IP + 2;
        let mut pt = IP + 3;
        let instruction = memory[IP];
        if (instruction % 1000) / 100 == 0 {
            p1 = memory[p1] as usize;
        }
        if (instruction % 10000) / 1000 == 0 {
            p2 = memory[p2] as usize;
        }
        if (instruction % 100_000) / 10000 == 0 {
            pt = memory[pt] as usize;
        }
        match instruction % 100 {
            1 => {
                memory[pt] = memory[p1] + memory[p2];
                IP += 4;
            }
            2 => {
                memory[pt] = memory[p1] * memory[p2];
                IP += 4;
            }
            3 => {
                memory[pt] = inputs.next().unwrap();
                IP += 2;
            }
            4 => {
                println!("{}", memory[p1]);
                IP += 2;
            }
            5 => {
                if memory[p1] != 0 {
                    IP = memory[p2] as usize;
                } else {
                    IP += 3;
                }
            }
            6 => {
                if memory[p1] == 0 {
                    IP = memory[p2] as usize;
                } else {
                    IP += 3;
                }
            }
            7 => {
                memory[pt] = if memory[p1] < memory[p2] { 1 } else { 0 };
                IP += 4;
            }
            8 => {
                memory[pt] = if memory[p1] == memory[p2] { 1 } else { 0 };
                IP += 4;
            }
            99 => {
                break;
            }
            _ => panic!("unknown operation {}", instruction),
        };
    }
}
