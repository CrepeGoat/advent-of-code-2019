use std::cmp::{min, max};
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

impl Coordinate {
	fn manhattan_dist(c1: &Coordinate, c2: &Coordinate) -> i64 {
		return (c1.x - c2.x).abs() + (c1.y - c2.y).abs()
	}
}


#[derive(Debug)]
struct LineSegment {
	p1: Coordinate,
	p2: Coordinate,
	min_score: i64,
}

impl LineSegment {
	pub fn xbounds(&self) -> (i64, i64) {
		if self.p1.x <= self.p2.x {
			return (self.p1.x, self.p2.x)
		} else {
			return (self.p2.x, self.p1.x)
		}
	}

	pub fn ybounds(&self) -> (i64, i64) {
		if self.p1.y <= self.p2.y {
			return (self.p1.y, self.p2.y)
		} else {
			return (self.p2.y, self.p1.y)
		}
	}

	pub fn intersection(s1: &LineSegment, s2: &LineSegment)
	-> Option<LineSegment> {
		let s1_xbounds = s1.xbounds();
		let s1_ybounds = s1.ybounds();
		let s2_xbounds = s2.xbounds();
		let s2_ybounds = s2.ybounds();

		if !(
			(s1_xbounds.0 <= s2_xbounds.1)
			& (s2_xbounds.0 <= s1_xbounds.1)
			& (s1_ybounds.0 <= s2_ybounds.1)
			& (s2_ybounds.0 <= s1_ybounds.1)
		) {
			return None;
		}

		let p1 = Coordinate {
			x: max(s1_xbounds.0, s2_xbounds.0),
			y: max(s1_ybounds.0, s2_ybounds.0),
		};
		let p2 = Coordinate {
			x: min(s1_xbounds.1, s2_xbounds.1),
			y: min(s1_ybounds.1, s2_ybounds.1),
		};

		let score =
			Coordinate::manhattan_dist(&p1, &s1.p1) + s1.min_score
			+ Coordinate::manhattan_dist(&p1, &s2.p1) + s2.min_score;

		return Some(LineSegment{p1:p1, p2:p2, min_score: score});
	}
}

fn as_segments(moves: &Vec<ManhattanMove>) -> Vec<LineSegment> {
	let mut coord0: Coordinate;
	let mut coord1 = Coordinate {x:0, y:0};
	let mut score = 0;

	let mut result = Vec::<LineSegment>::new();
	let mut intersections = Vec::<LineSegment>::new();

	let mut new_segment: LineSegment;

	for movement in moves.iter() {
		coord0 = coord1;

		match movement.orientation {
			Orientation::Vertical => coord1.y += movement.distance,
			Orientation::Horizontal => coord1.x += movement.distance,
		};

		new_segment = LineSegment{ p1:coord0, p2:coord1, min_score:score };

		score = new_segment.min_score + Coordinate::manhattan_dist(&new_segment.p1, &new_segment.p2);
		result.push(new_segment);
		intersections.clear();
	}

	result
}

//------------------------------------------------------------------

fn find_closest_intersection(
	path1: &Vec<LineSegment>,
	path2: &Vec<LineSegment>,
) -> Option<LineSegment> {
	let mut result: Option<LineSegment> = None;

	fn skip_if_suboptimal(
		segment_opt: Option<LineSegment>,
		res: &Option<LineSegment>
	) -> Option<LineSegment> {
		match (&segment_opt, &res)  {
			(_, None) => segment_opt,
			(None, _) => segment_opt,
			(Some(segment), Some(best_segment)) => {
				if segment.min_score < best_segment.min_score {
					segment_opt
				} else {
					None
				}
			}
		}
	};

	for seg1 in path1.iter() {
		for seg2 in path2.iter() {
			if let Some(segment) = skip_if_suboptimal(
				LineSegment::intersection(seg1, seg2), &result
			) {
				if segment.min_score > 0 {
					result = Some(segment);
				}
			}
		}
	}

	return result;
}


//------------------------------------------------------------------

fn main() {
	let mut movements = Vec::<ManhattanMove>::new();
	let mut buffer = String::new();

	let mut read_path = || -> Vec<LineSegment> {
		std::io::stdin().read_line(&mut buffer).expect("invalid path");
		parse_sequence(&mut movements, &buffer);
		let path_segments = as_segments(&movements);
		movements.clear();
		buffer.clear();

		path_segments
	};
	
	println!("Enter 1st wire path:");
	let path1_segments = read_path();

	println!("Enter 2nd wire path:");
	let path2_segments = read_path();

	let best_intersection = find_closest_intersection(
		&path1_segments, &path2_segments
	).unwrap();
	println!("closest intersection: {:?}", best_intersection);
	println!("distance: {:?}", best_intersection.min_score);
}
