use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
	let mut buf = [0u8; 11];
	let bytes_read = stream.read(&mut buf).unwrap();
	println!("{}", String::from_utf8_lossy(&buf));
}

fn main() {
	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	for stream in listener.incoming() {
		handle_client(stream.unwrap());
	}
}