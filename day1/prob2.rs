use std::vec::Vec;
use std::cmp::max;

pub fn fuel_req(mass: i64) -> i64 {
	return max(0, mass/3 - 2);
}

pub fn total_fuel_req<'a, T>(masses: T) -> i64
where
	T: std::iter::Iterator<Item=&'a i64>
{
	let mut total: i64 = 0;
	for mass in masses {
		let mut submass: i64 = fuel_req(*mass);
		let mut subtotal: i64 = 0;

		while submass > 0 {
			subtotal += submass;
			submass = fuel_req(submass);
		}

		total += subtotal;
	}

	return total;
}

// -----------------------------------------------------------

pub fn get_masses(list: &mut Vec<i64>) {
	let mut buffer: String;
	loop {
		buffer = String::new();

		match std::io::stdin().read_line(&mut buffer) {
		 	Ok(_) => match buffer.trim().parse::<i64>() {
		 		Ok(n) => list.push(n),
		 		Err(_) => break,
		 	},
		 	Err(_) => break,
		};
	}
}

pub fn main() {
	let mut masses = Vec::<i64>::new();
	get_masses(&mut masses);
	let fuel = total_fuel_req(masses.iter());

	println!("{:?}", fuel);
}