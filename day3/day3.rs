use std::vec::Vec;


//------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
struct ManhattanMove {
	direction: Direction,
	distance: u64,
}

impl ManhattanMove {
	fn from_str(s: &str) -> Result<ManhattanMove, &'static str> {
		let direction = match &s[..1] {
			"u" | "U" => Some(Direction::Up),
			"d" | "D" => Some(Direction::Down),
			"l" | "L" => Some(Direction::Left),
			"r" | "R" => Some(Direction::Right),
			_ => None,
		};
		let distance = &s[1..].parse::<u64>();

		return match (direction, distance) {
			(None, _dist) => Err("invalid direction"),
			(Some(ref _dir), Err(_e)) => Err("invalid distance"),
			(Some(ref dir), Ok(dist)) => Ok(ManhattanMove {
				direction: *dir,
				distance: *dist,
			}),
		}
	}
}

fn parse_sequence(output: &mut Vec<ManhattanMove>, input: &str) {
	let mut word = String::new();

	for ch in input.chars() {
		if ch == ',' {
			output.push(
				ManhattanMove::from_str(word.trim())
				.expect("invalid movement string")
			);
			word.clear();
		}
		else {
			word.push(ch);
		}
	}
	
	output.push(
		ManhattanMove::from_str(word.trim())
		.expect("invalid movement string")
	);
}

//------------------------------------------------------------------

fn main() {
	println!("Enter wire path:");
	let mut move_seq = Vec::<ManhattanMove>::new();
	let mut buffer = String::new();
	std::io::stdin().read_line(&mut buffer).expect("invalid code");
	parse_sequence(&mut move_seq, &buffer);

	for i in move_seq.iter() {
		println!("{:?}", i);
	}
}
