use std::io::{self, Write};
use std::net::TcpStream;

fn main(){
	let mut stream=TcpStream::connect("127.0.0.1:8080").unwrap();
    let mut input = String::new(); 
    loop {
        print!("To Server : ");
        io::stdout().flush().unwrap(); //added so that the prompt gets printed immeaditely
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    stream.write_all(input.as_bytes()).unwrap();
    }
    
}
