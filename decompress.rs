use std::io::{Read, Write}; //Reading from stdin

//TOBEORNOTTOBEORTOBEORNOT
//[84, 79, 66, 69, 79, 82, 78, 79, 84, 256, 258, 260, 265, 259, 261, 263]



//We need to accomodate dictionary sizes of 32 bits
fn decompress(data: &Vec<u32>) {
	
	let mut dict: Vec<Vec<u8>> = Vec::new();
	
	for size in 0u8..=255 {
		let mut vec: Vec<u8> = Vec::new();
		vec.push(size);
		dict.push(vec);
	}
	
	//.append() moves items in one vector to the other. We need to copy.
	
	let mut output: Vec<u8> = Vec::new();	
	
	let mut previous: Vec<u8> = Vec::new();
	previous.push(data[0] as u8); //The first code directly translates to the output byte
	output.append(&mut previous.clone());
	
	println!("{:?}", previous);
	println!("{:?}", output);
	println!("{:?}", data);

	
	//Start at one. First code is in
	for counter in 1..data.len() {
		
		let mut current: Vec<u8>;
		
		//Not in dictionary
		if counter == data.len() {
			current = previous.clone();
			current.push(previous[0]);
		}
		else {
			let code = data[counter];
			current = (dict[code as usize]).clone();
		}
		
		output.append(&mut current.clone());
		
		previous.push(current[0]);
		dict.push(previous);
		
		previous = Vec::new();
		previous.append(&mut current);
		
		
		
		println!("{:?}", output);
		
		
		
	}
	
	

	//println!("{:?}", dict);
	//println!("{:?}", output);
	println!("Finished! Output contains {} entries", output.len());
	
	
	
	
}






fn main() {
	//let mut stdin = std::io::stdin();
	//let mut data = Vec::with_capacity(1024);
	//stdin.read_to_end(&mut data).unwrap();
	//println!("Length of data: {}",data.len());
	let mut data: Vec<u32> = Vec::new();
	data.push(84);
	data.push(79);
	data.push(66);
	data.push(69);
	data.push(79);
	data.push(82);
	data.push(78);
	data.push(79);
	data.push(84);
	data.push(256);
	data.push(258);
	data.push(260);
	data.push(265);
	data.push(259);
	data.push(261);
	data.push(263);
	
	decompress(&mut data);
}




