use std::convert::{TryFrom, TryInto};
use std::cmp::min;
use std::ops::{RangeBounds, Bound::*};
use std::vec::Vec;


#[derive(Debug, Clone, Copy)]
struct Digits {
	value: u32
}

impl Digits {
	const NO_OF_DIGITS: u8 = 9;

	fn new(value: u32) -> Self {
		Digits{value: value}
	}

	fn subdigits<R: RangeBounds<u8>>(&self, index: R) -> Digits {
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

		Digits::new(
			if lbound >= ubound {
				0_u32
			} else {
				(self.value / 10_u32.pow(lbound.into()))
				% 10_u32.pow((ubound-lbound).into())
			}
		)
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

//-----------------------------------------------------------------------------

fn get_param_ref<'a>(
	program: &'a Vec<i32>, pos: usize, modes: Digits, offset: u8
) -> &'a i32 {
	ParameterRef
	::from_pos_mode(
		pos+1+usize::from(offset),
		modes.subdigits(2+offset..3+offset).value
	).unwrap()
	.deref(program).unwrap()
}

fn get_param_mutref<'a>(
	program: &'a mut Vec<i32>, pos: usize, modes: Digits, offset: u8
) -> &'a mut i32 {
	ParameterMutRef
	::from_pos_mode(
		pos+1+usize::from(offset),
		modes.subdigits(2+offset..3+offset).value
	).unwrap()
	.deref(program).unwrap()
}

//-----------------------------------------------------------------------------

fn exec_code(program: &mut Vec<i32>) {
	let mut pos = 0usize;

	while pos < program.len() {
		let op_modes = Digits::new(program[pos].try_into().unwrap());

		match OpInstruction::from_opcode(op_modes.subdigits(..2).value).unwrap() {
			OpInstruction::Add => {
				*get_param_mutref(program, pos, op_modes, 2)
				= get_param_ref(program, pos, op_modes, 0)
				+ get_param_ref(program, pos, op_modes, 1);

				pos += 4;
			}
			OpInstruction::Multiply => {
				*get_param_mutref(program, pos, op_modes, 2)
				= get_param_ref(program, pos, op_modes, 0)
				* get_param_ref(program, pos, op_modes, 1);

				pos += 4;
			}
			OpInstruction::Input => {
				println!("Enter in an input value");
				let mut buffer = String::new();
				std::io::stdin().read_line(&mut buffer).expect("invalid code");
				let input_value = buffer.trim().parse::<i32>().expect(
					"invalid input string"
				);

				*get_param_mutref(program, pos, op_modes, 0) = input_value;

				pos += 2;
			}
			OpInstruction::Output => {
				println!("{:?}", get_param_ref(program, pos, op_modes, 0));

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
	println!("Enter program code below:");
	let mut program = Vec::<i32>::new();
	let mut buffer = String::new();
	std::io::stdin().read_line(&mut buffer).expect("invalid program");
	parse_code_string(&mut program, &buffer);
	buffer.clear();
	
	exec_code(&mut program);
}
