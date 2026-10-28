use std::io::{stdout,stdin, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;

fn handle_client(stream: TcpStream) {
	let mut read_stream=stream.try_clone().unwrap();
	let mut write_stream=stream;
	let mut buf = [0u8; 1024];

	//reading thread
	thread::spawn(move || {
		loop{
			let bytes_read= read_stream.read(&mut buf).unwrap();
			if bytes_read==0{
				println!("Connection disconnected");
				std::process::exit(0);
			}
			if buf!=[0u8;1024]{
				print!("client: {}",String::from_utf8_lossy(&buf[..bytes_read]));
				buf=[0u8; 1024];
			}

		}
	});

	//writing thread
	let mut message=String::new();
	loop{
		stdout().flush().unwrap();
		stdin().read_line(&mut message).unwrap();
		if message!=""{
			print!("\x1B[1A\x1B[2K\r");
			stdout().flush().unwrap();
			print!("server: {}",message);
			write_stream.write_all(message.as_bytes()).unwrap();
			message.clear();
		}

	}

}

fn main() {
	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	println!("Server listening...");
	let aut_stream:TcpStream;

	loop{
		//establishing connection
		let (mut stream,addr) =listener.accept().unwrap();//returns Result<(TcpStream, SocketAddr), io::Error>
		stream.write_all(b"connected").unwrap();
		aut_stream=stream;
		println!("Connected Successfully!");
		println!("{}",addr);
		break;
	}

	//intruder detection
	let mut temp_stream=aut_stream.try_clone().unwrap();
	thread::spawn(move || {
		loop{
			let (stream,addr)= listener.accept().unwrap();
			if addr!=temp_stream.peer_addr().unwrap(){
				println!("Intruder detected");
				temp_stream.write_all(b"Intruder detected").unwrap();
			}
		}
	});

	handle_client(aut_stream);


}
