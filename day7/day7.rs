mod intcode_comp_d7;
use intcode_comp_d7::*;

use std::vec::Vec;
use std::iter::Iterator;


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


fn run_amp_chain<ITER>(program: Vec<i32>, phases: ITER, first_input: i32) -> i32
where ITER: Iterator<Item = u8>
{
	let mut last_output = first_input;
	for phase in phases {
		last_output = run_amplifier(program.clone(), phase, last_output);
	}

	last_output
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
