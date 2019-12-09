use std::vec::Vec;


//------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
enum Orientation {
	Horizontal, Vertical,
}

#[derive(Debug)]
struct ManhattanMove {
	orientation: Orientation,
	distance: i64,
}

impl ManhattanMove {
	fn from_str(s: &str) -> Result<ManhattanMove, &'static str> {
		let wrapped_dist = &s[1..].parse::<i64>();
		
		return match wrapped_dist {
			Err(_e) => Err("invalid distance"),
			Ok(abs_dist) => {
				let orient_str = &s[..1];
				let dist_vals = match orient_str {
					"u" | "U" => Some((Orientation::Vertical, *abs_dist)),
					"d" | "D" => Some((Orientation::Vertical, -*abs_dist)),
					"l" | "L" => Some((Orientation::Horizontal, -*abs_dist)),
					"r" | "R" => Some((Orientation::Horizontal, *abs_dist)),
					_ => None,
				};

				return match dist_vals {
					Some((orient, dist)) => Ok(ManhattanMove {
						orientation: orient,
						distance: dist,
					}),
					None => Err("invalid direction"),
				};
			},
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
