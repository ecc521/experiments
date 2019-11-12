use std::collections::HashMap;

fn main() {
	let mut dict = HashMap::new();
	
	dict.insert("Name", "Joseph");
	
    match dict.get(&"Name") {
        Some(&number) => println!("Name: {}", number),
        _ => println!("Name not available"),
    }


}