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
	for word in input.split(",") {
		output.push(
			ManhattanMove::from_str(word.trim())
			.expect("invalid movement string")
		);
	}
}

//------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
struct Coordinate {
	x: i64,
	y: i64,
}

#[derive(Debug)]
struct LineSegment (Coordinate, Coordinate);

fn as_segments(output: &mut Vec<LineSegment>, input: &Vec<ManhattanMove>) {
	let mut coord0: Coordinate;
	let mut coord1 = Coordinate {x:0, y:0};

	for movement in input.iter() {
		coord0 = coord1;

		match movement.orientation {
			Orientation::Vertical => coord1.y += movement.distance,
			Orientation::Horizontal => coord1.x += movement.distance,
		};

		output.push(LineSegment(coord0, coord1));
	}
}

//------------------------------------------------------------------

fn main() {
	println!("Enter wire path:");
	let mut movements = Vec::<ManhattanMove>::new();
	let mut buffer = String::new();
	std::io::stdin().read_line(&mut buffer).expect("invalid code");
	parse_sequence(&mut movements, &buffer);

	for i in movements.iter() {
		println!("{:?}", i);
	}

	let mut segments = Vec::<LineSegment>::new();
	as_segments(&mut segments, &movements);

	for i in segments.iter() {
		println!("{:?}", i);
	}
}
