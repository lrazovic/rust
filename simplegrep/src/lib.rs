use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub struct Config {
	pub query: String,
	pub file: String,
	pub case_sensitive: bool //export CASE_INSENSITIVE=1 in terminal
}

impl Config {
	pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
		if args.len() < 3 {
			eprintln!("USAGE: cargo run <query> <file>");
			process::exit(-1)
		}
		args.next();
		let query = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a query string"),
		};

		let file = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a file name"),
		};
		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

		Ok(Config {
			query,
			file,
			case_sensitive
		})
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.file)?;
	let result = if config.case_sensitive {
		search_sensitive(&config.query, &contents)
	} else {
		search_insensitive(&config.query, &contents)
	};
	for line in result {
		println!("{}", line);
	}
	Ok(())
}

pub fn search_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents
		.lines()
		.filter(|line| line.contains(query))
		.collect()
}

fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents
		.lines()
		.filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn case_sensitive() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
		assert_eq!(
			vec!["safe, fast, productive."],
			search_sensitive(query, contents)
		);
	}
	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(
			vec!["Rust:", "Trust me."],
			search_insensitive(query, contents)
		);
	}
}
