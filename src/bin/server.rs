use std::io::Read;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
	let mut buf = [0u8; 1024];
	loop {
        let bytes_read = stream.read(&mut buf).unwrap();

        if bytes_read == 0 {
            println!("Client disconnected");
            break;
        }

        print!("From Client: {}",String::from_utf8_lossy(&buf[..bytes_read])); //print only the read bytes, not the extra spaces
}
}

fn main() {
	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	println!("Server listening...");
		loop {
        let (stream, addr) = listener.accept().unwrap(); //returns Result<(TcpStream, SocketAddr), io::Error>
        println!("Client connected: {}", addr);

        handle_client(stream);
    }
	}
