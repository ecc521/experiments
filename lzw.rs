use std::io::Read; //Reading from stdin
use std::collections::HashMap; //LZW dictionary
//The default hashing algorithm may be slow. Consider changing it



//TOBEORNOTTOBEORTOBEORNOT
//[84, 79, 66, 69, 79, 82, 78, 79, 84, 256, 258, 260, 265, 259, 261, 263]






fn compress(data: &mut [u8]) {
	
	//We need to accomodate dictionary sizes of 32 bits
	let mut dict = HashMap::<String, u32>::new();
	let mut size: u32 = 255; //Start at 0

	//Initialize dictionary
	for size in 0u8..=255 {
		//size as char
		dict.insert((size as char).to_string(), size as u32);
	}
	
	
	//.push to add to vector
	
	//This will use much more memory than is needed, but shoudn't be an issue
	let mut output: Vec<u32> = Vec::new();
		
	let mut previous = (data[0] as char).to_string();
	let mut word: String;	
		
	//Begin at one, since position 0 is the variable previous
	for counter in 1..data.len() {
		
		let current = (data[counter] as char).to_string();
		
		word = previous; //Consider changing the dict.get so that it uses a substring of word instead
		//That may be faster
		
		word.push_str(&current);
		
		if dict.contains_key(&word) {
			previous = word;
		}
		else {
			//Output the code for the previous value
			//The previous value is always word minus one character
			//UTF-8 has to ruin everything.. Including making this operation O(n)
			match dict.get(&word[0..(word.len() - word.chars().last().unwrap().to_string().len())]) {
				Some(&number) => output.push(number),
				_ => panic!("Error in HashMap (ignore last character) {}", word),
			}
			
			size += 1;
			//Add the new value to the dictionary
			dict.insert(word, size);
			previous = current;
		}	
		
		
		if counter%262144 == 0 {
			println!("{}", counter)
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




