use simplegrep::Config;
use std::env;
use std::process;

/*
1. Calling the command line parsing logic with the argument values
2. Setting up any other configuration
3. Calling a run function in lib.rs
4. Handling the error if run returns an error
*/

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(-1);
    });
    if let Err(err) = simplegrep::run(config) {
		eprintln!("Application error: {}", err);
		process::exit(-1)
	}
}
