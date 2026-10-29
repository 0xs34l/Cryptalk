use std::io::{stdout,stdin, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use ed25519_dalek::{ Signer, SigningKey, Signature, VerifyingKey, Verifier};
use getrandom::{SysRng, rand_core::{UnwrapErr}};

fn handle_client(stream: TcpStream, secret_key: SigningKey,sender_public_key: VerifyingKey) {
	let mut read_stream=stream.try_clone().unwrap();
	let mut write_stream=stream;

	//reading thread
	thread::spawn(move || {
		loop{
			let mut len_buf = [0u8; 4];

            if read_stream.read_exact(&mut len_buf).is_err() {
				println!("Connection disconnected");
                break;
            }

            let msg_len = u32::from_be_bytes(len_buf) as usize;

            if msg_len > 10 * 1024 * 1024 {
                println!("Message too large");
                break;
            }

            let mut msg_buf = vec![0u8; msg_len];

            if read_stream.read_exact(&mut msg_buf).is_err() {
                println!("Failed to read message");
                break;
            }

             let mut signature_buf = [0u8; 64];

            if read_stream.read_exact(&mut signature_buf).is_err() {
                println!("Failed to read signature");
                break;
            }

            let signature = Signature::from_bytes(&signature_buf);

            if sender_public_key.verify(&msg_buf, &signature).is_err() {
                println!("Invalid signature!");
                 continue;
            }

            print!("client: {}", String::from_utf8_lossy(&msg_buf));

		}
	});

	//writing thread
	let mut message=String::new();
	loop{
		stdout().flush().unwrap();
		if stdin().read_line(&mut message).unwrap() == 0 {
			 break;}
		if !message.trim().is_empty(){
			print!("\x1B[1A\x1B[2K\r");
			stdout().flush().unwrap();
			print!("server: {}",message);
			let message_bytes = message.as_bytes();
            let signature = secret_key.sign(message_bytes);
			let len=(message_bytes.len() as u32).to_be_bytes();
			write_stream.write_all(&len).unwrap();
			write_stream.write_all(message_bytes).unwrap();
			write_stream.write_all(&signature.to_bytes()).unwrap();
		} 
		message.clear();

	}

}

fn main() {
	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
	println!("Server listening...");

	let (aut_stream, secret_key, sender_public_key) = loop {
    let (mut stream, addr) = listener.accept().unwrap();
    stream.write_all(b"Connected").unwrap();

    let mut sender_key_bytes = [0u8; 32];
    stream.read_exact(&mut sender_key_bytes).unwrap();

    let sender_public_key =
        VerifyingKey::from_bytes(&sender_key_bytes).unwrap();

    let mut csprng = UnwrapErr(SysRng);
    let secret_key = SigningKey::generate(&mut csprng);

    let public_key = secret_key.verifying_key();
    stream.write_all(public_key.as_bytes()).unwrap();

    println!("Connected Successfully!");
    println!("{}", addr);

    break (stream, secret_key, sender_public_key);
};

	handle_client(aut_stream, secret_key, sender_public_key);


}
