use std::io;

fn main() {
	println!("Please input some text");
	
	let mut text = String::new();
	
	io::stdin().read_line(&mut text)
		.expect("Error reading line");
		
	println!("You input {}", text);
	println!("That is {} characters", text.len())
	
}