mod intcode_comp_d7;
use intcode_comp_d7::*;

use std::vec::Vec;
use std::iter::Iterator;
use std::default::Default;


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

trait TieredCounters<N> {
	const size: usize;
	fn increment(&mut self) -> N;
}


struct EmptyCounter();
struct TierCounter<T, U: TieredCounters<T>>(T, const T, U);

impl<T: Default> TieredCounters<T> for EmptyCounter {
	const size: usize = 0usize;
	fn increment(&mut self) -> T {T::default()}
}

impl<T, U: TieredCounters<T>, n: T> TieredCounters<T> for TierCounter<T, U> {
	const size: usize = 1+U::size;
	fn increment(&mut self) -> T {
		self.0 += self.2.increment();
		self.0 %= self.1
	}
}


type OneSlotCounter = TierCounter<u8, EmptyCounter, 5>;
type TwoSlotCounter = TierCounter<u8, OneSlotCounter, 5>;
type ThreeSlotCounter = TierCounter<u8, TwoSlotCounter, 5>;
type FourSlotCounter = TierCounter<u8, ThreeSlotCounter, 5>;
type FiveSlotCounter = TierCounter<u8, FourSlotCounter, 5>;


//-----------------------------------------------------------------------------

fn main() {
	println!("Enter program code below:");
	let mut program = Vec::<i32>::new();
	let mut buffer = String::new();
	std::io::stdin().read_line(&mut buffer).expect("invalid program");
	parse_code_string(&mut program, &buffer);
	buffer.clear();
	
	intcode_comp_d7::exec_program_over_stdio(program);
}
