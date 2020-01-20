mod intcode_comp_d7;
use intcode_comp_d7::*;

use std::mem::swap;
use std::vec::Vec;
use std::iter::Iterator;


fn run_amplifier(program: Vec<i32>, phase: u8, input_signal: i32) -> i32 {
	let program_state = match YieldStates::new(program) {
		YieldStates::Pause(ps) => ps,
		_ => panic!()
	};
	let program_state = match program_state.execute().unwrap() {
		YieldStates::Input(ps) => ps,
		_ => panic!()
	};
	let program_state = match program_state.execute(phase.into()).unwrap() {
		YieldStates::Input(ps) => ps,
		_ => panic!()
	};
	let program_state = match program_state.execute(input_signal).unwrap() {
		YieldStates::Output(ps) => ps,
		_ => panic!()
	};

	*program_state.get()
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


//-----------------------------------------------------------------------------

struct Permutation<n: usize> {
	perm: mut [T; n],
}

impl Iterator for Permutation<n: usize> {
	type Item = &[T; n];

	pub fn next(&mut self) -> Option<Self::Item> {
		let cycle_count = self.count / Self::n;
		let cycle_pos = self.count % Self::n;

		let i_switch = if cycle_count % 2 == 0 {
			cycle_pos % (Self::n - 1)
		} else {
			(Self::n - 1) - (cycle_pos % (Self::n - 1))
		}

		swap(&self.perm[i_switch], &self.perm[i_switch+1])
		self.count += 1;
		
	}
}



fn find_best_phases(program: Vec<i32>) -> {

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
