use std::io::{Write, stdout, stdin, Read};
use std::net::TcpStream;
use std::thread;

fn handle_server(stream:TcpStream){
	let mut write_stream:TcpStream=stream.try_clone().unwrap();
	let mut read_stream=stream;
	let mut buf=[0u8;1024];
	let temp_buf=[0u8;1024];

	//reading thread
	thread::spawn(move || {
		loop{
			let bytes_read=read_stream.read(&mut buf).unwrap();
			if bytes_read==0 {
				println!("Connection disconnected");
				std::process::exit(0);
			}
			if buf!=temp_buf{
				print!("server: {}",String::from_utf8_lossy(&buf[..bytes_read]));
			}
			buf=[0u8;1024];
		}
	});

	//writing thread
	let mut message=String::new();
	loop{
		stdout().flush().unwrap();
		stdin().read_line(&mut message).unwrap();
		if message!="".to_string(){
			print!("\x1B[1A\x1B[2K\r");
			stdout().flush().unwrap();
			print!("client: {}",message);
			write_stream.write_all(message.as_bytes()).unwrap();
			message.clear();
		}
	}
}

fn main(){
	let mut stream:TcpStream;
	loop{
		let temp=TcpStream::connect("127.0.0.1:8080");
		if temp.is_ok(){
			stream=temp.unwrap();
			break;
		}
	}
	loop{
		let mut buffer=[0u8;1024];
		let bytes_read=stream.read(&mut buffer[..1024]).unwrap();
		if bytes_read==9 {
			println!("Connected Successfully!");
			break;
		}
	}
	handle_server(stream);
    
}
