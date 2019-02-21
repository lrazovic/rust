use std::env;
use std::process;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() == 1 || args.len() == 2 {
		eprintln!("USAGE: cargo run <query> <file>");
		process::exit(-1)
	}
	let query = &args[1];
	let file = &args[2];
	print!("Searching {:?} ",query);
	println!("in File {}", file);
}