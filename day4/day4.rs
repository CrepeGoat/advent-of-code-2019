use std::convert::TryFrom;


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


fn parse_range_pair(input: &str) -> Result<(SixDigits, SixDigits), &str>
{
	let mut num_str_iter = input.split("-");
	let r1 = num_str_iter.next();
	let r2 = num_str_iter.next();

	if let (Some(str1), Some(str2)) = (r1, r2) {
		if let (Ok(val1), Ok(val2)) = (
			str1.trim().parse::<u32>(),
			str2.trim().parse::<u32>()
		) {
			Ok((SixDigits::new(val1), SixDigits::new(val2)))
		} else {
			Err("invalid numbers present")
		}
	} else {
		Err("requires two values in string, separated by a dash (-)")
	}
}


fn main() {
	let mut buffer = String::new();
	println!("Enter range pair:");
	std::io::stdin().read_line(&mut buffer).expect("stdin error");
	let range_pair = parse_range_pair(&buffer).expect("invalid range pair");

	println!("{:?}", range_pair);
	for i in 0..6 {
		print!("{}", range_pair.0.digit(i).unwrap());
	}
	println!("");
}