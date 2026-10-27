use ed25519_dalek::{ Signer, SigningKey, Verifier};
use getrandom::{SysRng, rand_core::{UnwrapErr}};
fn main() {
	let mut csprng=UnwrapErr(SysRng);
	let secret_key=SigningKey::generate(&mut csprng);
	let message="hello";
	let signature=secret_key.sign(message.as_bytes());
	let public_key=secret_key.verifying_key();
	let flag=public_key.verify(message.as_bytes(),&signature).is_ok();
	println!("{flag}");

}