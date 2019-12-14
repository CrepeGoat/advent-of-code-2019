#[derive(Debug, Clone, Copy)]
struct SixDigits(u32);


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
			Ok((SixDigits(val1), SixDigits(val2)))
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
}