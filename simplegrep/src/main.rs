use std::env;
use std::process;
use simplegrep::Config;

/*
1. Calling the command line parsing logic with the argument values
2. Setting up any other configuration
3. Calling a run function in lib.rs
4. Handling the error if run returns an error
*/

fn main() {
	let args: Vec<String> = env::args().collect();
	let config = Config::new(&args);
	print!("Searching {:?} ", config.query);
	println!("in File {}", config.file);
	if let Err(e) = simplegrep::run(config) {
		println!("Application error: {}", e);
		process::exit(-1)
	}
}
