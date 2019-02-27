use std::error::Error;
use std::fs;
use std::process;

pub struct Config {
	pub query: String,
	pub file: String,
}

impl Config {
	pub fn new(args: &[String]) -> Config {
		if args.len() < 3 {
			eprintln!("USAGE: cargo run <query> <file>");
			process::exit(-1)
		}
		let query = args[1].clone();
		let file = args[2].clone();
		Config { query, file }
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.file)?;

	println!("With text:\n{}", contents);

	Ok(())
}
