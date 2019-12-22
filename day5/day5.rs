use std::convert::{TryFrom, TryInto};
use std::cmp::min;
use std::ops::{RangeBounds, Bound::*};
use std::vec::Vec;


#[derive(Debug, Clone, Copy)]
struct Digits(u32);

impl Digits {
	const NO_OF_DIGITS: u8 = 9;

	fn digits<R: RangeBounds<u8>>(&self, index: R) -> u32 {
		let lbound = match index.start_bound() {
			Unbounded => 0,
			Included(&n) => n,
			Excluded(&n) => n+1
		};
		let ubound = match index.end_bound() {
			Unbounded => Self::NO_OF_DIGITS,
			Included(&n) => min(Self::NO_OF_DIGITS, n+1),
			Excluded(&n) => min(Self::NO_OF_DIGITS, n),
		};

		if lbound >= ubound {
			0_u32
		} else {
			(self.0 / 10_u32.pow(lbound.into()))
			% 10_u32.pow((ubound-lbound).into())
		}
	}
}

//-----------------------------------------------------------------------------

fn parse_code_string(output: &mut Vec<i32>, input: &str) {
	for word in input.split(",") {
		output.push(word.trim().parse::<i32>().expect("invalid input string"));
	}
}

//-----------------------------------------------------------------------------

#[derive(Debug)]
enum ParameterRef {
	Position(usize),
	Immediate(usize),
}

impl ParameterRef {
	fn from_pos_mode(pos: usize, mode: u32) -> Result<ParameterRef, String> {
		match mode {
			0 => Ok(Self::Position(pos)),
			1 => Ok(Self::Immediate(pos)),
			m => Err(format!("invalid parameter mode {:?}", m))
		}
	}
	fn deref<'a>(&self, program: &'a Vec<i32>) -> Result<&'a i32, String> {
		
		fn get_ref_value<'b>(program: &'b Vec<i32>, pos: usize, err: &str)
		-> Result<&'b i32, String> {
			program.get(pos).ok_or(format!("{:?} {:?}", err, pos))
		}

		match self {
			Self::Immediate(pos) => get_ref_value(
				program, *pos, 
				"invalid program position"
			),
			Self::Position(pos) => {
				if let Ok(pos_val) = usize::try_from(*get_ref_value(
					program, *pos,
					"invalid program position"
				)?) {
					get_ref_value(
						program, pos_val,
						"invalid position parameter"
					)					
				} else {
					panic!("program integer cannot be converted to position pointer")
				}
			},
		}
	}	
}

#[derive(Debug)]
enum ParameterMutRef {
	Position(usize),
}

impl ParameterMutRef {
	fn from_pos_mode(pos: usize, mode: u32) -> Result<ParameterMutRef, String> {
		match mode {
			0 => Ok(Self::Position(pos)),
			1 => Err("cannot mutably access a parameter in immediate mode".to_string()),
			m => Err(format!("invalid parameter mode {:?}", m))
		}
	}
	fn deref<'a>(&self, program: &'a mut Vec<i32>) -> Result<&'a mut i32, String> {
		let Self::Position(pos) = self;

		if let Ok(pos_val) = usize::try_from(*program.get(*pos).ok_or(
			format!("invalid program position {:?}", pos)
		)?) {
			program.get_mut(pos_val).ok_or(
				format!("invalid position parameter {:?}", pos_val)
			)
		} else {
			panic!("program integer cannot be converted to position pointer")
		}
	}	
}


#[derive(Debug)]
enum OpInstruction {
	Add,
	Multiply,
	Input,
	Output,

	Terminate,
}

impl OpInstruction {
	fn from_opcode(opcode: u32) -> Result<OpInstruction, String> {
		match opcode {
			99u32 => Ok(Self::Terminate),
			1u32 => Ok(Self::Add),
			2u32 => Ok(Self::Multiply),
			3u32 => Ok(Self::Input),
			4u32 => Ok(Self::Output),
			n => Err(format!("invalid opcode '{:?}'", n))
		}
	}
}

fn exec_code(program: &mut Vec<i32>) {
	let mut pos = 0usize;

	while pos < program.len() {
		let op_modes = Digits(program[pos].try_into().unwrap());

		match OpInstruction::from_opcode(op_modes.digits(..2)).unwrap() {
			OpInstruction::Add => {
				*ParameterMutRef
					::from_pos_mode(pos+3, op_modes.digits(4..5)).unwrap()
					.deref(program).unwrap()
				= ParameterRef
					::from_pos_mode(pos+1, op_modes.digits(2..3)).unwrap()
					.deref(program).unwrap()
				+ ParameterRef
					::from_pos_mode(pos+2, op_modes.digits(3..4)).unwrap()
					.deref(program).unwrap();

				pos += 4;
			}
			OpInstruction::Multiply => {
				*ParameterMutRef
					::from_pos_mode(pos+3, op_modes.digits(4..5)).unwrap()
					.deref(program).unwrap()
				= ParameterRef
					::from_pos_mode(pos+1, op_modes.digits(2..3)).unwrap()
					.deref(program).unwrap()
				* ParameterRef
					::from_pos_mode(pos+2, op_modes.digits(3..4)).unwrap()
					.deref(program).unwrap();

				pos += 4;
			}
			OpInstruction::Input => {
				println!("Enter in an input value");
				let mut buffer = String::new();
				std::io::stdin().read_line(&mut buffer).expect("invalid code");
				let input_value = buffer.trim().parse::<i32>().expect(
					"invalid input string"
				);

				*ParameterMutRef
					::from_pos_mode(pos+1, op_modes.digits(2..3)).unwrap()
					.deref(program).unwrap()
				= input_value;

				pos += 2;
			}
			OpInstruction::Output => {
				let output_value = ParameterRef
					::from_pos_mode(pos+1, op_modes.digits(2..3)).unwrap()
					.deref(program).unwrap();
				println!("{:?}", output_value);

				pos += 2;
			}
			OpInstruction::Terminate => break,
		}
	}
}

//------------------------------------------------------------------
fn print_code(code: &Vec<i32>) {
	let iter = code.iter();

	for ch in iter {
		print!("{:?},", ch);
	}

	println!("");
}

fn main() {
	let d = Digits(123456780);

	println!("{:?}", d.digits(1..=2))

	/*
	println!("Enter program code below:");
	let mut code = Vec::<usize>::new();
	let mut buffer = String::new();
	std::io::stdin().read_line(&mut buffer).expect("invalid code");
	parse_code_string(&mut code, &buffer);
	//exec_code(&mut code);
	buffer.clear();
	
	println!("Enter expected result at address 0:");
	std::io::stdin().read_line(&mut buffer).expect("no expected result found");
	let expected_result = buffer.trim().parse::<usize>().expect("invalid expected result");
	let desired_input = find_noun_verb(&code, expected_result);

	println!("{:?}{:?}", desired_input[0], desired_input[1]);
	*/
}