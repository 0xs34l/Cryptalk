use std::{io::Write, net::TcpStream};

fn main(){
	let mut stream=TcpStream::connect("127.0.0.1:8080").unwrap();
    let message=b"Hello world";
    stream.write_all(message).unwrap();
    println!("message sent");
}
