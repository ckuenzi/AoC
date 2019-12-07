use std::collections::VecDeque;

#[derive(Clone)]
pub struct Computer {
    pub id: i32,
    memory: Vec<i32>,
    original_memory: Vec<i32>,
    ip: usize,
    input: VecDeque<i32>,
    output: VecDeque<i32>,
    pub halted: bool,
    waiting_for_input: bool,
}

impl Computer {
    pub fn new(program: Vec<i32>) -> Computer {
        Computer {
            id: 0,
            memory: program.clone(),
            original_memory: program,
            ip: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            halted: false,
            waiting_for_input: false,
        }
    }

    pub fn step(&mut self) {
        let mut p1 = self.ip + 1;
        let mut p2 = self.ip + 2;
        let mut pt = self.ip + 3;
        self.wrap_pointer(&mut p1);
        self.wrap_pointer(&mut p2);
        self.wrap_pointer(&mut pt);
        let instruction = self.memory[self.ip];
        if (instruction % 1000) / 100 == 0 {
            p1 = self.memory[p1] as usize;
        }
        if (instruction % 10000) / 1000 == 0 {
            p2 = self.memory[p2] as usize;
        }
        if (instruction % 100_000) / 10000 == 0 {
            pt = self.memory[pt] as usize;
        }
        match instruction % 100 {
            1 => {
                self.memory[pt] = self.memory[p1] + self.memory[p2];
                self.ip += 4;
            }
            2 => {
                self.memory[pt] = self.memory[p1] * self.memory[p2];
                self.ip += 4;
            }
            3 => {
                if let Some(next_input) = self.input.pop_front() {
                    self.memory[p1] = next_input;
                    self.ip += 2;
                } else {
                    self.waiting_for_input = true;
                }
            }
            4 => {
                //println!("{}", self.memory[p1]);
                self.output.push_back(self.memory[p1]);
                self.ip += 2;
            }
            5 => {
                if self.memory[p1] != 0 {
                    self.ip = self.memory[p2] as usize;
                } else {
                    self.ip += 3;
                }
            }
            6 => {
                if self.memory[p1] == 0 {
                    self.ip = self.memory[p2] as usize;
                } else {
                    self.ip += 3;
                }
            }
            7 => {
                self.memory[pt] = if self.memory[p1] < self.memory[p2] {
                    1
                } else {
                    0
                };
                self.ip += 4;
            }
            8 => {
                self.memory[pt] = if self.memory[p1] == self.memory[p2] {
                    1
                } else {
                    0
                };
                self.ip += 4;
            }
            99 => {
                self.halted = true;
            }
            _ => panic!("unknown operation {}", instruction),
        };
    }

    #[allow(dead_code)]
    pub fn run(&mut self) {
        self.waiting_for_input = false;
        while !self.halted && !self.waiting_for_input {
            self.step();
        }
    }

    #[allow(dead_code)]
    pub fn siso(&mut self, input: i32) -> i32 {
        self.add_input(input);
        self.run();
        self.get_output()
            .unwrap_or_else(|| panic!("SISO no output available"))
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.memory = self.original_memory.clone();
        self.ip = 0;
        self.input = VecDeque::new();
        self.output = VecDeque::new();
        self.halted = false;
        self.waiting_for_input = false;
    }

    #[allow(dead_code)]
    pub fn add_inputs(&mut self, input: Vec<i32>) {
        self.input.append(&mut VecDeque::from(input));
    }

    #[allow(dead_code)]
    pub fn add_input(&mut self, input: i32) {
        self.input.push_back(input);
    }

    #[allow(dead_code)]
    pub fn get_output(&mut self) -> Option<i32> {
        self.output.pop_front()
    }

    #[allow(dead_code)]
    pub fn get_outputs(&mut self) -> VecDeque<i32> {
        self.output.clone()
    }

    fn wrap_pointer(&self, p: &mut usize) {
        if p >= &mut self.memory.len() {
            *p = p.wrapping_sub(self.memory.len());
        }
    }
}

pub fn read_intscript_from_file(file: &str) -> Vec<i32> {
    file.split(',').map(|x| x.parse::<i32>().unwrap()).collect()
}
