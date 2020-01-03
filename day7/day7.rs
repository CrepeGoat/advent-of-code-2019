mod intcode_comp_d7;
use intcode_comp_d7::*;

use std::vec::Vec;


fn main() {
	println!("Enter program code below:");
	let mut program = Vec::<i32>::new();
	let mut buffer = String::new();
	std::io::stdin().read_line(&mut buffer).expect("invalid program");
	parse_code_string(&mut program, &buffer);
	buffer.clear();
	
	intcode_comp_d7::exec_program_over_stdio(program);
}
