use std::io::{self, Write, stdout, stdin, Read};
use std::net::TcpStream;

fn main(){
	let mut stream=TcpStream::connect("127.0.0.1:8080").unwrap();
    let mut input = String::new();
	loop{
		let mut buffer=[0u8;1024];
		let bytes_read=stream.read(&mut buffer[..1024]).unwrap();
		if(bytes_read==9) {
			println!("Connected Successfully!");
			break;
		}
	}
    loop {
        print!("To Server : ");
        stdout().flush().unwrap(); //added so that the prompt gets printed immediately
	    input.clear();
	    stdin().read_line(&mut input).unwrap();
	    stream.write_all(input.as_bytes()).unwrap();
    }
    
}
