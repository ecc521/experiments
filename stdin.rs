    use std::io::Read;


fn main() {
        let mut stdin = std::io::stdin();
        let mut data = Vec::with_capacity(1024);
        stdin.read_to_end(&mut data).unwrap();
        println!("{}",data.len());
    }