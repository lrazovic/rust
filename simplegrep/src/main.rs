use std::env;
use std::fs;
use std::process;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() == 1 || args.len() == 2 {
		eprintln!("USAGE: cargo run <query> <file>");
		process::exit(-1)
	}
	let query = &args[1];
	let file = &args[2];
	let contents = fs::read_to_string(file).expect("Something went wrong reading the file");
	print!("Searching {:?} ", query);
	println!("in File {}", file);
	println!("With text:\n{}", contents);
}
