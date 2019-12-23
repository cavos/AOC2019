use std::fs::File;
use std::io::{BufRead, BufReader};

use super::interpreter::IntcodeInterpreter;

pub fn solve(input_file: &str) {
    let mut intcode = read_input(input_file);
    intcode[1] = 12;
    intcode[2] = 2;

    let mut computer = IntcodeInterpreter::new(&intcode);
    computer.execute();

    let result = computer.read_memory(0).unwrap();
    assert!(result == 4462686);
    println!("Day 02.1: intcode at 0: {:}", result)
}

pub fn solve_pt2(input_file: &str, expected_val: i32) {
    let intcode = read_input(input_file);

    let mut noun = 0i32;
    let mut verb = 0i32;
    while noun <= 99 && verb <= 99 {
        let mut computer = IntcodeInterpreter::new(&intcode);
        computer.set_memory(1, noun as i128);
        computer.set_memory(2, verb as i128);
        computer.execute();

        if expected_val == computer.read_memory(0).unwrap() as i32 {
            break;
        }
        verb += 1;
        if verb > 99 {
            verb = 0;
            noun += 1;
        }
    }

    assert!(noun == 59);
    assert!(verb == 36);
    println!(
        "Day 02.2: noun {:}, verb {:}, answer {:}",
        noun,
        verb,
        100 * noun + verb
    );
}

fn read_input(input_file: &str) -> Vec<i128> {
    let mut buffered = BufReader::new(File::open(input_file).unwrap());
    let mut input_data = String::new();
    let r = buffered.read_line(&mut input_data);
    if r.is_err() {
        panic!("Failed to read input data!")
    }
    input_data
        .split(',')
        .map(|c| c.parse::<i128>().unwrap())
        .collect()
}
