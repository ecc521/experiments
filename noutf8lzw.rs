use std::io::Read; //Reading from stdin
use std::collections::HashMap; //LZW dictionary
//The default hashing algorithm may be slow. Consider changing it



//TOBEORNOTTOBEORTOBEORNOT
//[84, 79, 66, 69, 79, 82, 78, 79, 84, 256, 258, 260, 265, 259, 261, 263]






fn compress(data: &mut [u8]) {
	
	//We need to accomodate dictionary sizes of 32 bits
	let mut dict = HashMap::<Vec<u8>, u32>::new();
	let mut size: u32 = 255; //Start at 0

	//Initialize dictionary
	for size in 0u8..=255 {
		let mut vec: Vec<u8> = Vec::new();
		vec.push(size);
		dict.insert(vec, size as u32);
	}
	
	
	//.push to add to vector
	
	//This will use much more memory than is needed, but shoudn't be an issue
	let mut output: Vec<u32> = Vec::new();
		
	let mut previous: Vec<u8> = Vec::new();
	previous.push(data[0]);
	
	let mut word: Vec<u8>;	
		
	//Begin at one, since position 0 is the variable previous
	for counter in 1..data.len() {
		
		let current = data[counter];
		
		word = previous; 
		word.push(current);
		
		if dict.contains_key(&word) {
			previous = word;
		}
		else {
			//Output the code for the previous value
			//Reference was lost for previous
			//Simply use word and ignore last character
			match dict.get(&word[0..(word.len()-1)]) {
				Some(&number) => output.push(number),
				_ => panic!("Error in HashMap (Ignore last byte) {:?}", word),
			}
			
			size += 1;
			//Add the new value to the dictionary
			dict.insert(word, size);
			//Set previous to current
			previous = Vec::<u8>::new();
			previous.push(current);
		}	
	}
	
	

	
	match dict.get(&previous) {
		Some(&number) => output.push(number),
		_ => panic!("Error in HashMap")
	}
	
	
	
	//println!("{:?}", dict);
	//println!("{:?}", output)
	println!("Finished! Output contains {} entries", output.len())
	
	
	
	
}






fn main() {
	let mut stdin = std::io::stdin();
	let mut data = Vec::with_capacity(1024);
	stdin.read_to_end(&mut data).unwrap();
	println!("Length of data: {}",data.len());
	compress(&mut data)
}




