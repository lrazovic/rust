use std::env;
use std::fs;
use std::process;

/*
1. Calling the command line parsing logic with the argument values
2. Setting up any other configuration
3. Calling a run function in lib.rs
4. Handling the error if run returns an error
*/

struct Config {
	query: String,
	file: String,
}

fn main() {
	let args: Vec<String> = env::args().collect();
	usage(&args);
	let config = parse_config(&args);
	print!("Searching {:?} ", config.query);
	println!("in File {}", config.file);
	let contents = fs::read_to_string(config.file).expect("Something went wrong reading the file");
	println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) -> Config {
	let query = args[1].clone();
	let file = args[2].clone();
	Config {query, file}
}

fn usage(args: &[String]) {
	let len = args.len();
	if len == 1 || len == 2 {
		eprintln!("USAGE: cargo run <query> <file>");
		process::exit(-1)
	}
}
