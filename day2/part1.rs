use std::vec::Vec;

fn exec_code(code: &mut Vec<usize>) {
	let mut code_pos = 0usize;

	while code_pos < code.len() {
		let op = code[code_pos];
		if op == 99usize {
			break;
		}

		let arg1pos = code[code_pos+1];
		let arg2pos = code[code_pos+2];
		let respos = code[code_pos+3];

		if op == 1usize {
			code[respos] = code[arg1pos] + code[arg2pos]
		}
		else if op == 2usize {
			code[respos] = code[arg1pos] * code[arg2pos]
		}
		else {
			panic!("invalid opcode {:?}", op);
		}

		code_pos += 4;
	}
}

//------------------------------------------------------------------

fn parse_code_string(output: &mut Vec<usize>, input: &str) {
	let mut word = String::new();

	for ch in input.chars() {
		if ch == ',' {
			output.push(
				word.trim().parse::<usize>().expect(
					"invalid input string"
				)
			);
			word.clear();
		}
		else {
			word.push(ch);
		}
	}
	
	output.push(
		word.trim().parse::<usize>().expect(
			"invalid input string"
		)
	);
}

fn print_code(code: &Vec<usize>) {
	let iter = code.iter();

	/*match iter.next() {
		Some(expr) => print!("{:?}", expr),
		None => {},
	}
	*/
	for ch in iter {
		print!("{:?},", ch);
	}

	println!("");
}

fn main() {
	println!("Enter program code below:");

	let mut code = Vec::<usize>::new();
	let mut buffer = String::new();
	std::io::stdin().read_line(&mut buffer).expect("invalid code");
	
	parse_code_string(&mut code, &buffer);
	exec_code(&mut code);

	print_code(&code);
}