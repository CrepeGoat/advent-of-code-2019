use std::convert::{TryFrom, TryInto};
use std::cmp::{min, Ordering};
use std::ops::{RangeBounds, Bound::*};
use std::vec::Vec;


#[derive(Debug, Clone, Copy)]
struct Digits(u32);

impl Digits {
	const NO_OF_DIGITS: u8 = 9;

	pub fn subdigits<R: RangeBounds<u8>>(&self, index: R) -> Self {
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

		Self::from(
			if lbound >= ubound {
				0_u32
			} else {
				(self.0 / 10_u32.pow(lbound.into()))
				% 10_u32.pow((ubound-lbound).into())
			}
		)
	}
}

impl From<u32> for Digits {
	fn from(value: u32) -> Self {
		Self(value)
	}
}

impl Into<u32> for Digits {
	fn into(self) -> u32 {
		self.0
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
enum ErrorCode {
	ParamMode(u32),  // parameter mode is invalid
	ProgramPosition(usize),  // invalid position in program
	PositionValue(i32),  // invalid value at position in program
}


#[derive(Debug)]
struct ParameterRef{op_pos: usize, offset: u8}

impl ParameterRef {
	fn deref<'a>(&self, program: &'a Vec<i32>) -> Result<&'a i32, ErrorCode> {
		let mode: u32 = {
			let op_value = *program.get(self.op_pos)
				.ok_or(ErrorCode::ProgramPosition(self.op_pos))?;
			let op_code: u32 = op_value.try_into()
				.ok().ok_or(ErrorCode::PositionValue(op_value))?;
			
			Digits::from(op_code).subdigits(2+self.offset..3+self.offset).into()
		};
		let param_pos = self.op_pos+1+usize::from(self.offset);

		match mode {
			0 => {
				let value = *program.get(param_pos).ok_or(
					ErrorCode::ProgramPosition(param_pos)
				)?;
				let value_pos = usize::try_from(value).ok().ok_or(
					ErrorCode::PositionValue(value)
				)?;

				program.get(value_pos).ok_or(
					ErrorCode::ProgramPosition(value_pos)
				)
			},
			1 => program.get(param_pos).ok_or(
				ErrorCode::ProgramPosition(param_pos)
			),
			m => Err(ErrorCode::ParamMode(m))
		}
	}	
}

#[derive(Debug)]
struct ParameterMutRef{op_pos: usize, offset: u8}

impl ParameterMutRef {
	fn deref<'a>(&self, program: &'a mut Vec<i32>) -> Result<&'a mut i32, ErrorCode> {
		let mode: u32 = {
			let op_value = *program.get(self.op_pos)
				.ok_or(ErrorCode::ProgramPosition(self.op_pos))?;
			let op_code: u32 = op_value.try_into()
				.ok().ok_or(ErrorCode::PositionValue(op_value))?;
			
			Digits::from(op_code).subdigits(2+self.offset..3+self.offset).into()
		};
		let param_pos = self.op_pos+1+usize::from(self.offset);

		match mode {
			0 => {
				let value = *program.get(param_pos).ok_or(
					ErrorCode::ProgramPosition(param_pos)
				)?;
				let value_pos = usize::try_from(value).ok().ok_or(
					ErrorCode::PositionValue(value)
				)?;

				program.get_mut(value_pos).ok_or(
					ErrorCode::ProgramPosition(value_pos)
				)
			},
			m => Err(ErrorCode::ParamMode(m))
		}
	}
}


fn get_param_ref<'a>(
	program: &'a Vec<i32>, op_pos: usize, offset: u8
) -> &'a i32 {
	ParameterRef{op_pos, offset}.deref(program).unwrap()
}

fn get_param_mutref<'a>(
	program: &'a mut Vec<i32>, op_pos: usize, offset: u8
) -> &'a mut i32 {
	ParameterMutRef{op_pos, offset}.deref(program).unwrap()
}

//-----------------------------------------------------------------------------

#[derive(Debug)]
enum OpInstruction {
	Add,
	Multiply,
	Input,
	Output,
	Jump(bool),
	Compare(Ordering),

	Terminate,
}

impl OpInstruction {
	fn from_opcode(opcode: u32) -> Result<OpInstruction, String> {
		match Digits::from(opcode).subdigits(..2).into() {
			99u32 => Ok(Self::Terminate),
			1u32 => Ok(Self::Add),
			2u32 => Ok(Self::Multiply),
			3u32 => Ok(Self::Input),
			4u32 => Ok(Self::Output),
			5u32 => Ok(Self::Jump(true)),
			6u32 => Ok(Self::Jump(false)),
			7u32 => Ok(Self::Compare(Ordering::Less)),
			8u32 => Ok(Self::Compare(Ordering::Equal)),
			n => Err(format!("invalid opcode '{:?}'", n))
		}
	}
}

//-----------------------------------------------------------------------------

struct YieldStartInner {
	program: Vec<i32>
}

impl YieldStartInner {
	fn execute(self) -> YieldStates {
		exec_program(self.program, 0)
	}
}

struct YieldInputInner {
	program: Vec<i32>,
	start_pos: usize,
	write_ref: ParameterMutRef
}

impl YieldInputInner {
	fn execute(mut self, input_value: i32) -> YieldStates {
		*self.write_ref.deref(&mut self.program).unwrap() = input_value;
		exec_program(self.program, self.start_pos)
	}
}

struct YieldOutputInner {
	program: Vec<i32>,
	start_pos: usize,
	read_ref: ParameterRef
}

impl YieldOutputInner {
	fn get<'a>(&'a self) -> &'a i32 {
		self.read_ref.deref(&self.program).unwrap()
	}
	fn execute(self) -> YieldStates {
		exec_program(self.program, self.start_pos)
	}
}


enum YieldStates {
	Start(YieldStartInner),
	Input(YieldInputInner),
	Output(YieldOutputInner),
	Stop,
}

impl YieldStates {
	fn new(program: Vec<i32>) -> Self {
		Self::Start(YieldStartInner{program: program})
	}
}


fn exec_program(mut program: Vec<i32>, start_pos: usize) -> YieldStates {
	let mut pos = start_pos;

	while pos < program.len() {
		let op_modes = u32::try_from(program[pos]).unwrap();

		match OpInstruction::from_opcode(op_modes).unwrap() {
			OpInstruction::Add => {
				*get_param_mutref(&mut program, pos, 2)
				= get_param_ref(&program, pos, 0)
				+ get_param_ref(&program, pos, 1);

				pos += 4;
			}
			OpInstruction::Multiply => {
				*get_param_mutref(&mut program, pos, 2)
				= get_param_ref(&program, pos, 0)
				* get_param_ref(&program, pos, 1);

				pos += 4;
			}
			OpInstruction::Input => {
				return YieldStates::Input(YieldInputInner{
					program, start_pos: pos+2,
					write_ref: ParameterMutRef{op_pos: pos, offset: 0}
				});
			}
			OpInstruction::Output => {
				return YieldStates::Output(YieldOutputInner{
					program, start_pos: pos+2,
					read_ref: ParameterRef{op_pos: pos, offset: 0}
				});
			}
			OpInstruction::Jump(trigger) => {
				if trigger == (0 !=
					*get_param_ref(&program, pos, 0)
				) {
					pos = usize::try_from(
						*get_param_ref(&program, pos, 1)
					).unwrap();
				} else {
					pos += 3;
				}
			}
			OpInstruction::Compare(trigger) => {
				*get_param_mutref(&mut program, pos, 2)
					= (trigger == get_param_ref(&program, pos, 0).cmp(
						get_param_ref(&program, pos, 1)
					)) as i32;

				pos += 4;
			}
			OpInstruction::Terminate => return YieldStates::Stop,
		}
	}

	YieldStates::Stop
}

fn exec_program_over_stdio(program: Vec<i32>) {
	let mut program_state = YieldStates::new(program);
	
	use YieldStates::*;
	loop {
		program_state = match program_state {
			Start(process) => process.execute(),
			Input(process) => {
				println!("Enter an input value:");
				let mut buffer = String::new();
				std::io::stdin().read_line(&mut buffer).expect("invalid code");
				let input_value = buffer.trim().parse::<i32>().expect(
					"invalid input string"
				);

				process.execute(input_value)
			}
			Output(process) => {
				println!("{:?}", process.get());

				process.execute()
			}
			Stop => break,
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
	
	exec_program_over_stdio(program);
}
