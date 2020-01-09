mod intcode_comp_d7;
use intcode_comp_d7::*;

use std::vec::Vec;


fn run_amplifier(program: Vec<i32>, phase: u8, input_signal: i32) -> i32 {
	if let YieldStates::Pause(program_state) = YieldStates::new(program) {
		if let YieldStates::Input(program_state) = program_state.execute().unwrap() {
			if let YieldStates::Input(program_state) = program_state.execute(phase.into()).unwrap() {
				if let YieldStates::Output(program_state) = program_state.execute(input_signal).unwrap() {
					return *program_state.get();
				}
			}
		}
	}
	panic!();
}


fn main() {
	println!("Enter program code below:");
	let mut program = Vec::<i32>::new();
	let mut buffer = String::new();
	std::io::stdin().read_line(&mut buffer).expect("invalid program");
	parse_code_string(&mut program, &buffer);
	buffer.clear();
	
	intcode_comp_d7::exec_program_over_stdio(program);
}
