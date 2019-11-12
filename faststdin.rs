    use std::io::Read;


fn main() {
        let mut stdin = std::io::stdin();
		//We'll read up to 65536 bytes at a time
        let mut data = [0u8; 65536];
        let mut len = 0;
        while let Ok(n) = stdin.read(&mut data) {
			//No more data! End loop
            if n == 0 {
                break;
            }
			//n is the amount of bytes that were written into data
			//Compression should be run on the data
        	//println!("{}", len);
            len += n;
        }   
        println!("{}", len);
    }