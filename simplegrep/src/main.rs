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
	if let Err(e) = simplegrep::run(config) {
		eprintln!("Application error: {}", e);
		process::exit(-1)
	}
}
