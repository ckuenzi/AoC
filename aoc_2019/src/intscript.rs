use hashbrown::HashMap;
use std::collections::VecDeque;
use itertools::Itertools;

#[derive(Clone)]
pub struct Computer {
    pub id: i64,
    memory: Vec<i64>,
    extra_memory: HashMap<usize, i64>,
    original_memory: Vec<i64>,
    ip: usize,
    relative_base: i64,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    pub halted: bool,
    waiting_for_input: bool,
}

impl Computer {
    pub fn new(program: Vec<i64>) -> Computer {
        Computer {
            id: 0,
            memory: program.clone(),
            original_memory: program,
            extra_memory: HashMap::new(),
            ip: 0,
            relative_base: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            halted: false,
            waiting_for_input: false,
        }
    }

    #[allow(dead_code)]
    pub fn new_from_text(input: &str) -> Computer {
        let memory = read_intscript_from_file(input);
        Computer::new(memory)
    }

    fn get_pointers(&mut self, offset: i64) -> usize {
        let p = self.ip + offset as usize;
        let instruction = self.read(self.ip);
        match (instruction % (10_i64.pow(offset as u32) * 100)) / (10_i64.pow(offset as u32) * 10) {
            0 => self.read(p) as usize,
            1 => p,
            2 => (self.read(p) + self.relative_base) as usize,
            _ => panic!("{} unknown mode", instruction),
        }
    }

    pub fn step(&mut self) {
        let instruction = self.read(self.ip);
        let p1 = self.get_pointers(1);
        let p2 = self.get_pointers(2);
        let pt = self.get_pointers(3);
        match instruction % 100 {
            1 => {
                let res = self.read(p1) + self.read(p2);
                self.write(pt, res);
                self.ip += 4;
            }
            2 => {
                let res = self.read(p1) * self.read(p2);
                self.write(pt, res);
                self.ip += 4;
            }
            3 => {
                if let Some(next_input) = self.input.pop_front() {
                    self.write(p1, next_input);
                    self.ip += 2;
                } else {
                    self.waiting_for_input = true;
                }
            }
            4 => {
                //println!("{}", self.read(p1));
                let out = self.read(p1);
                self.output.push_back(out);
                self.ip += 2;
            }
            5 => {
                if self.read(p1) != 0 {
                    self.ip = self.read(p2) as usize;
                } else {
                    self.ip += 3;
                }
            }
            6 => {
                if self.read(p1) == 0 {
                    self.ip = self.read(p2) as usize;
                } else {
                    self.ip += 3;
                }
            }
            7 => {
                let res = (self.read(p1) < self.read(p2)) as i64;
                self.write(pt, res);
                self.ip += 4;
            }
            8 => {
                let res = (self.read(p1) == self.read(p2)) as i64;
                self.write(pt, res);
                self.ip += 4;
            }
            9 => {
                self.relative_base += self.read(p1);
                self.ip += 2;
            }
            99 => {
                self.halted = true;
            }
            _ => panic!("unknown operation {}", instruction),
        };
    }

    #[allow(dead_code)]
    pub fn run(&mut self) -> &mut Computer {
        self.waiting_for_input = false;
        while !self.halted && !self.waiting_for_input {
            self.step();
        }
        self
    }

    #[allow(dead_code)]
    pub fn siso(&mut self, input: i64) -> i64 {
        self.add_input(input);
        self.run();
        self.get_output()
            .unwrap_or_else(|| panic!("SISO no output available"))
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) -> &mut Computer {
        self.memory = self.original_memory.clone();
        self.extra_memory = HashMap::new();
        self.ip = 0;
        self.relative_base = 0;
        self.input = VecDeque::new();
        self.output = VecDeque::new();
        self.halted = false;
        self.waiting_for_input = false;
        self
    }

    #[allow(dead_code)]
    pub fn add_inputs(&mut self, input: Vec<i64>) -> &mut Computer {
        self.input.append(&mut VecDeque::from(input));
        self
    }

    #[allow(dead_code)]
    pub fn add_input(&mut self, input: i64) -> &mut Computer {
        self.input.push_back(input);
        self
    }

    #[allow(dead_code)]
    pub fn get_output(&mut self) -> Option<i64> {
        self.output.pop_front()
    }

    #[allow(dead_code)]
    pub fn get_outputs(&mut self) -> VecDeque<i64> {
        self.output.clone()
    }
    #[allow(dead_code)]
    pub fn dump_outputs(&mut self) -> Vec<i64> {
        let out = Vec::from(self.output.clone());
        self.output.clear();
        out
    }

    pub fn write(&mut self, address: usize, value: i64) -> &mut Computer {
        if address < self.memory.len() {
            self.memory[address] = value;
        } else {
            *self.extra_memory.entry(address).or_insert(value) = value;
        }
        self
    }

    pub fn read(&mut self, address: usize) -> i64 {
        match address < self.memory.len() {
            true => self.memory[address],
            false => *self.extra_memory.entry(address).or_insert(0),
        }
    }

    pub fn output_char(&mut self) -> Option<char> {
        if let Some(i) = self.get_output() {
            return Some(char::from(i as u8));
        } else {
            return None;
        }
    }

    pub fn output_string(&mut self) -> String {
        self.dump_outputs()
            .iter()
            .map(|&c| char::from(c as u8))
            .fold(String::new(), |out, c| format!("{}{}", out, c))
    }

    pub fn input_string(&mut self, input: &str){
        self.add_inputs(input.as_bytes().iter().map(|&a| a as i64).collect_vec());
    }
}

pub fn read_intscript_from_file(file: &str) -> Vec<i64> {
    file.split(',').map(|x| x.parse::<i64>().unwrap()).collect()
}
