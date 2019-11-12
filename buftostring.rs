use std::io::Read; //Reading from stdin
use std::collections::HashMap; //LZW dictionary
//The default hashing algorithm may be slow. Consider changing it














fn compress(data: &mut [u8]) {
	
	let mut str = String::new();
	
	for counter in 0..data.len() {
		str.push(data[counter] as char);
	}
		
	println!("String has : {} characters. Buffer has {} bytes.", str.len(), data.len());
	println!("{}", str)
}	
	
	
	
	
	
	
	



//dict.contains_key

	//dict.insert("Name", "Joseph");
	
    //match dict.get(&"Name") {
    //    Some(&number) => println!("Name: {}", number),
    //    _ => println!("Name not available"),
    //}



fn main() {
	let mut stdin = std::io::stdin();
	let mut data = Vec::with_capacity(1024);
	stdin.read_to_end(&mut data).unwrap();
	println!("Length of data: {}",data.len());
	compress(&mut data)
}






