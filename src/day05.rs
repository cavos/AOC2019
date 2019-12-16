use super::interpreter::IntcodeInterpreter;

pub fn solve(input_file : &str) {
    let mut interpreter = IntcodeInterpreter::from_file(input_file);
    interpreter.set_input(1);
    interpreter.execute();

    let output = interpreter.get_output();
    assert!(output.back() == Some(&12234644));
    println!("Day 05.1: Output is {:?}", output);
}

pub fn solve_pt2(input_file : &str) {
    let mut interpreter = IntcodeInterpreter::from_file(input_file);
    interpreter.set_input(5);

    interpreter.execute();

    let output = interpreter.get_output();
    assert!(output.back() == Some(&3508186));
    println!("Day 05.2: Output is {:?}", output);
}
