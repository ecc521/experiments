use std::collections::HashMap;

fn main() {
	let mut map = HashMap::new();
	let mut vec: Vec<u8> = Vec::new();
	vec.push(221);
	map.insert(vec.clone(), "bob");
	println!("{}", map.contains_key(&vec));
}