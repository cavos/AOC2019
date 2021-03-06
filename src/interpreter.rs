use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Clone, Debug)]
pub enum ExecutionState {
    Working,
    Suspended,
    Finished,
}

#[derive(Clone)]
pub struct IntcodeInterpreter {
    memory: Vec<i128>,
    instr_ptr: usize,
    state: ExecutionState,
    input: VecDeque<i32>,
    output: VecDeque<i32>,
}

impl IntcodeInterpreter {
    pub fn new(instuctions: &Vec<i128>) -> IntcodeInterpreter {
        IntcodeInterpreter {
            memory: instuctions.clone(),
            instr_ptr: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            state: ExecutionState::Working,
        }
    }

    pub fn from_file(input_file: &str) -> IntcodeInterpreter {
        let mut buffered = BufReader::new(File::open(input_file).unwrap());
        let mut input_data = String::new();
        let r = buffered.read_line(&mut input_data);
        if r.is_err() {
            panic!("Failed to read input data!")
        }
        let instructions = input_data
            .split(',')
            .map(|c| c.parse::<i128>().unwrap())
            .collect();

        IntcodeInterpreter {
            memory: instructions,
            instr_ptr: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            state: ExecutionState::Working,
        }
    }

    pub fn read_memory(&self, address: usize) -> Option<i128> {
        let r = self.memory.get(address);

        if r.is_some() {
            return Some(*r.unwrap());
        }
        None
    }

    pub fn get_state(&self) -> &ExecutionState {
        &self.state
    }

    pub fn set_input(&mut self, input: i32) {
        self.input.push_back(input);
        // println!("{:?}", self.input);
    }

    pub fn get_output(&mut self) -> Option<i32> {
        self.output.pop_front()
    }

    pub fn set_memory(&mut self, address: usize, value: i128) {
        self.memory[address] = value;
    }

    pub fn execute(&mut self) -> ExecutionState {
        self.state = ExecutionState::Working;
        while self.state == ExecutionState::Working {
            let instr_code = self.memory[self.instr_ptr] % 100;
            let arg_modes = format!("{:010}", (self.memory[self.instr_ptr] / 100) as i32);

            // print!("[pc:{}][{}] instr:{}, modes:{}",self.instr_ptr,self.memory[self.instr_ptr],instr_code, arg_modes );

            match instr_code {
                1 => self.execute_addition(arg_modes.as_str()),
                2 => self.execute_multiplication(arg_modes.as_str()),
                3 => self.state = self.read_input(),
                4 => self.write_output(arg_modes.as_str()),
                5 => self.jmp_if_true(arg_modes.as_str()),
                6 => self.jmp_if_false(arg_modes.as_str()),
                7 => self.less_than(arg_modes.as_str()),
                8 => self.equal(arg_modes.as_str()),
                99 => self.state = ExecutionState::Finished,
                _ => panic!("Unsupported instruction!"),
            }
        }

        self.state.clone()
    }

    fn read_arg_value(&self, pos: u32, modes: &str) -> i128 {
        let a = self.memory[self.instr_ptr + pos as usize];
        // print!("<{} = {}>", modes.chars().rev().nth((pos-1) as usize).unwrap(), pos);
        if modes.chars().rev().nth((pos - 1) as usize).unwrap() == '0' {
            // print!(" mem val at {}, ", a);
            return self.memory[a as usize];
        }
        // print!(" immediate val {}, ", a);
        a
    }

    fn execute_addition(&mut self, arg_modes: &str) {
        // let a1 = self.memory[self.instr_ptr + 1];
        // let a2 = self.memory[self.instr_ptr + 2];
        let o = self.memory[self.instr_ptr + 3];

        // let v1 = if arg_modes%10==0 {self.memory[a1 as usize]} else {a1};
        // let v2 = if arg_modes%100==0 {self.memory[a2 as usize]} else {a2};
        let v1 = self.read_arg_value(1, arg_modes);
        let v2 = self.read_arg_value(2, arg_modes);

        // println!("{} = {} + {}", o, v1, v2 );
        self.memory[o as usize] = v1 + v2;
        self.instr_ptr += 4;
    }

    fn execute_multiplication(&mut self, arg_modes: &str) {
        let v1 = self.read_arg_value(1, arg_modes);
        let v2 = self.read_arg_value(2, arg_modes);
        let o = self.memory[self.instr_ptr + 3];

        // println!("{} = {} * {}", o, v1, v2 );
        self.memory[o as usize] = v1 * v2;
        self.instr_ptr += 4;
    }

    fn read_input(&mut self) -> ExecutionState {
        let r = self.memory[self.instr_ptr + 1];
        // println!("read input at addr {}", r);
        let input = self.input.pop_front();
        if input.is_none() {
            return ExecutionState::Suspended;
        }
        self.memory[r as usize] = input.unwrap() as i128;
        self.instr_ptr += 2;
        ExecutionState::Working
    }

    fn write_output(&mut self, arg_modes: &str) {
        // let r = self.memory[self.instr_ptr + 1];
        let r = self.read_arg_value(1, arg_modes);

        self.output.push_back(r as i32);
        self.instr_ptr += 2;
        // println!(" output value => {}", r);
    }

    fn jmp_if_true(&mut self, arg_modes: &str) {
        let r = self.read_arg_value(1, arg_modes);
        if r != 0 {
            self.instr_ptr = self.read_arg_value(2, arg_modes) as usize;
        } else {
            self.instr_ptr += 3;
        }
    }

    fn jmp_if_false(&mut self, arg_modes: &str) {
        let r = self.read_arg_value(1, arg_modes);
        if r == 0 {
            self.instr_ptr = self.read_arg_value(2, arg_modes) as usize;
        } else {
            self.instr_ptr += 3;
        }
    }

    fn less_than(&mut self, arg_modes: &str) {
        let r1 = self.read_arg_value(1, arg_modes);
        let r2 = self.read_arg_value(2, arg_modes);
        let o = self.memory[self.instr_ptr + 3];

        self.memory[o as usize] = if r1 < r2 { 1 } else { 0 };
        self.instr_ptr += 4;
    }

    fn equal(&mut self, arg_modes: &str) {
        let r1 = self.read_arg_value(1, arg_modes);
        let r2 = self.read_arg_value(2, arg_modes);
        let o = self.memory[self.instr_ptr + 3];

        self.memory[o as usize] = if r1 == r2 { 1 } else { 0 };
        self.instr_ptr += 4;
    }
}
