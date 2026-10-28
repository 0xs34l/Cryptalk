use std::io::{stdout,stdin, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::os::linux::raw::ino_t;
use std::thread;

fn handle_client(mut stream: TcpStream,listener: TcpListener) {
	let mut read_stream=stream.try_clone().unwrap();
	let mut write_stream=stream;
	let mut buf = [0u8; 1024];
	let temp_addr:SocketAddr=read_stream.peer_addr().unwrap();
	//reading
	thread::spawn(move || {
		loop{
			//detecting intruder
			let (int_stream,int_addr)= listener.accept().unwrap();
			if(int_addr==temp_addr){
				println!("Intruder detected");
				read_stream.write_all(b"Intruder detected").unwrap();
			}


			let bytes_read= read_stream.read(&mut buf).unwrap();
			if bytes_read==0{
				return;
			}
			if buf!=[0u8;1024]{
				println!("Client:{}",String::from_utf8_lossy(&buf[..1024]));
				buf=[0u8; 1024];
			}

		}
	});

	//writing
	let mut message=String::new();
	thread::spawn(move || {
		loop{
			stdout().flush().unwrap();
			stdin().read_line(&mut message).unwrap();
			if message!=""{
				println!("server:{}",message);
				write_stream.write_all(message.as_bytes()).unwrap();
				message=String::new();
			}

		}
	});

}

fn main() {
	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	println!("Server listening...");

	loop{
		//establishing connection
		let (mut stream,addr) =listener.accept().unwrap();//returns Result<(TcpStream, SocketAddr), io::Error>
		stream.write_all(b"connected").unwrap();
		println!("Connected Successfully!");
		println!("{}",addr);
		handle_client(stream,listener);
		break;
	}

}
