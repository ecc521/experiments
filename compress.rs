use std::io::{Read, Write}; //Reading from stdin
use std::collections::HashMap; //LZW dictionary
//The default hashing algorithm may be slow. Consider changing it


//use std::mem; //Converting &u32 to four &u8
//It requires unsafe, though is likely faster than a naive solution



//TOBEORNOTTOBEORTOBEORNOT
//[84, 79, 66, 69, 79, 82, 78, 79, 84, 256, 258, 260, 265, 259, 261, 263]






fn compress(data: &mut [u8]) {
	
	//We need to accomodate dictionary sizes of 32 bits
	let mut dict = HashMap::<Vec<u8>, u32>::new();
	let mut size: u32 = 256; //Starts at 0

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
				_ => panic!("Error in HashMap (Ignore last bwriteyte) {:?}", word),
			}
			
			size += 1;
			//Add the new value to the dictionary
			dict.insert(word, size);
			//Set previous to current
			previous = Vec::<u8>::new();
			previous.push(current);
		}
		
		if counter % 262144 == 0 {
			println!("Bytes processed: {}", counter);
		}
	}
	
	

	
	match dict.get(&previous) {
		Some(&number) => output.push(number),
		_ => panic!("Error in HashMap")
	}
	
	
	
	//println!("{:?}", dict);
	//println!("{:?}", output)
	println!("Finished! Output contains {} entries", output.len());
	
	//let mut stdout = std::io::stdout();
	//let u32arr = &*output; //Convert to &u32 from Vec<u32>
	//let u8arr;

	//Convert &u32 to &u8
	//unsafe {
	//	u8arr = mem::transmute::<Vec<u32>, [u8; output.len() * 4]>(output);
		
	//}
	
	//stdout.write(&u8arr);
	
	
	
}






fn main() {
	let mut stdin = std::io::stdin();
	let mut data: Vec<u8> = Vec::with_capacity(1024);
	stdin.read_to_end(&mut data).unwrap();
	println!("Length of data: {}",data.len());
	compress(&mut data)
}




