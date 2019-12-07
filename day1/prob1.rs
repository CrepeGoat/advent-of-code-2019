use std::vec::Vec;
use std::cmp::max;

pub fn fuel_req(mass: u64) -> u64 {
	return max(0, mass/3 - 2);
}

pub fn total_fuel_req<'a, T>(masses: T) -> u64
where
	T: std::iter::Iterator<Item=&'a u64>
{
	let mut total: u64 = 0;
	for mass in masses {
		total += fuel_req(*mass);
	}

	return total;
}

// -----------------------------------------------------------

pub fn get_masses(list: &mut Vec<u64>) {
	let mut buffer: String;
	loop {
		buffer = String::new();

		match std::io::stdin().read_line(&mut buffer) {
		 	Ok(_) => match buffer.trim().parse::<u64>() {
		 		Ok(n) => list.push(n),
		 		Err(_) => break,
		 	},
		 	Err(_) => break,
		};
	}
}

pub fn main() {
	let mut masses = Vec::<u64>::new();
	get_masses(&mut masses);
	let fuel = total_fuel_req(masses.iter());

	println!("{:?}", fuel);
}