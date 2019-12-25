use std::vec::Vec;
use std::collections::HashMap;


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

//-----------------------------------------------------------------------------

#[derive(Debug)]
struct UpLink {
	parent: Option<String>,
	depth: u32,
}

fn read_orbits(uplinks: &mut HashMap<String, UpLink>) {
	fn push_orbit(
		uplinks: &mut HashMap<String, UpLink>,
		downlinks: &mut HashMap<String, Vec<String>>,
		parent: &String, child: &String,
	) {
		if let Some(parent_depth) = uplinks.get(parent).map(|x| x.depth) {
			load_orbit(uplinks, downlinks, parent, child, parent_depth);
		}
		else if let Some(dl_chain) = downlinks.get_mut(parent) {
			dl_chain.push(child.clone());
		} else {
			downlinks.insert(parent.clone(), vec![child.clone()]);
		}
	}

	fn load_orbit(
		uplinks: &mut HashMap<String, UpLink>,
		downlinks: &mut HashMap<String, Vec<String>>,
		parent: &String, child: &String, parent_depth: u32,
	) {
		uplinks.insert(child.clone(), UpLink{
			parent: Some(parent.clone()), depth: parent_depth+1
		});
		load_downlinks(uplinks, downlinks, child, parent_depth+1);
	}

	fn load_downlinks(
		uplinks: &mut HashMap<String, UpLink>,
		downlinks: &mut HashMap<String, Vec<String>>,
		parent: &String, parent_depth: u32,
	) {
		if let Some(dependents) = downlinks.remove(parent) {
			for child in dependents.iter() {
				load_orbit(uplinks, downlinks, parent, child, parent_depth);
			}
		}
	}

	let mut read_buffer = String::new();
	let mut downlinks_buffer = HashMap::<String, Vec<String>>::new();

	uplinks.insert("COM".to_string(), UpLink{parent: None, depth: 0});

	while 
		std::io::stdin().read_line(&mut read_buffer).is_ok()
		&& !read_buffer.trim().is_empty()
	{
		let orbit_pair = parse_orbit(&read_buffer).unwrap();
		push_orbit(uplinks, &mut downlinks_buffer, &orbit_pair.0, &orbit_pair.1);

		read_buffer.clear();
	}
}

fn main() {
	let mut uplinks = HashMap::<String, UpLink>::new();
	println!("Enter orbit pairs:");
	read_orbits(&mut uplinks);

	/*
	println!(
		"Number of orbits: {:?}",
		uplinks.values().map(|x| x.depth).sum::<u32>()
	);
	*/
}
