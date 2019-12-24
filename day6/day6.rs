fn parse_orbit(input: &str) -> Result<(String, String), &str> {
	let mut str_iter = input.trim().split(")");
	let p1 = str_iter.next().ok_or(
		"requires two values in string, separated by a right-parenthesis"
	)?;
	let p2 = str_iter.next().ok_or(
		"requires two values in string, separated by a right-parenthesis"
	)?; 

	match str_iter.next() {
		None => Ok((p1.to_string(), p2.to_string())),
		Some(_) => Err("invalid orbital pair format"),
	}
}

