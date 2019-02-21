use std::env;
use std::fs;

fn main() {
	let args: Vec<String> = env::args().collect();
	let query = &args[1];
	let file = &args[2];
	print!("Searching {:?} ",query);
	println!("in File {}", file);
}