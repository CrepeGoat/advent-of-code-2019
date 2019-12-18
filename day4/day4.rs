use std::convert::{From, TryFrom};
use std::cmp::Ordering;
use std::ops::RangeInclusive;


#[derive(Debug, Clone, Copy)]
struct SixDigits(u32);

impl SixDigits {
	fn new(value: u32) -> SixDigits {
		assert!((0..10_u32.pow(6)).contains(&value));
		SixDigits(value)
	}

	fn digit(&self, idx: usize) -> Result<u8, String> {
		if idx < 6 {
			Ok(u8::try_from(
				(self.0 / 10_u32.pow(u32::try_from(idx).unwrap())) % 10
			).unwrap())
		} else {
			Err(String::from("index out of bounds"))
		}
	}
}


fn parse_range(input: &str) -> Result<RangeInclusive<u32>, &str>
{
	let mut num_str_iter = input.split("-");
	let r1 = num_str_iter.next();
	let r2 = num_str_iter.next();

	if let (Some(str1), Some(str2)) = (r1, r2) {
		if let (Ok(val1), Ok(val2)) = (
			str1.trim().parse::<u32>(),
			str2.trim().parse::<u32>()
		) {
			Ok(RangeInclusive::new(val1, val2))
		} else {
			Err("invalid numbers present")
		}
	} else {
		Err("requires two values in string, separated by a dash (-)")
	}
}

//-----------------------------------------------------------------------------

fn is_valid_password(value: u32) -> bool {
	let digits = SixDigits::new(value);
	let mut has_repeat = false;
	for i in 0..5 {
		match digits.digit(i).cmp(&digits.digit(i+1)) {
			Ordering::Less => return false,
			Ordering::Equal => has_repeat = true,
			_ => (),
		};
	}
	return has_repeat;
}


fn count_passwords<T: std::iter::Iterator<Item=u32>>(iter: T) -> u32 {
	iter.map(|x| u32::from(is_valid_password(x))).sum()
}

//-----------------------------------------------------------------------------

fn is_valid_password2(value: u32) -> bool {
	let digits = SixDigits::new(value);
	let mut has_repeat = false;
	let mut repeat_streak = 1;
	for i in 0..5 {
		match digits.digit(i).cmp(&digits.digit(i+1)) {
			Ordering::Less => {
				return false;
			},
			Ordering::Equal => {
				repeat_streak += 1;
			},
			Ordering::Greater => {
				has_repeat |= repeat_streak == 2;
				repeat_streak = 1;
			},
		};
	}
	return has_repeat | (repeat_streak == 2);
}


fn count_passwords2<T: std::iter::Iterator<Item=u32>>(iter: T) -> u32 {
	iter.map(|x| u32::from(is_valid_password2(x))).sum()
}

//-----------------------------------------------------------------------------

fn main() {
	let mut buffer = String::new();
	println!("Enter range pair:");
	std::io::stdin().read_line(&mut buffer).expect("stdin error");
	let range = parse_range(&buffer).expect("invalid range pair");

	println!("{:?}", range);
	for i in 0..6 {
		print!("{}", SixDigits::new(*range.start()).digit(i).unwrap());
	}
	println!("");

	println!("valid passwords in range: {:?}", count_passwords2(range));
}