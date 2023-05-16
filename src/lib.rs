use std::error::Error;
use std::fs;

const RED: &'static str = "\u{001B}[31;1m";
const RESET: &'static str = "\u{001B}[0m";

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(config.filename)?;

	let results = if config.case_sensitive {
		search(&config.query, &contents)
	} else {
		search_case_insensitive(&config.query, &contents)
	};
	let updated_value = format!("{}{}{}", RED, &config.query, RESET);
	for line in results {
		let value = line.replace(&config.query, &updated_value);
		println!("{value}");
	}

	return Ok(());
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let mut results = Vec::new();
	for line in contents.lines() {
		if line.contains(query) {
			results.push(line);
		}
	}

	return results;
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut results = Vec::new();
	for line in contents.lines() {
		if line.to_lowercase().contains(&query) {
			results.push(line);
		}
	}

	return results;
}

pub struct Config {
	pub query: String,
	pub filename: String,
	pub case_sensitive: bool,
}

impl Config {
	pub fn new(args: &Vec<String>) -> Result<Self, &str> {
		if args.len() < 3 {
			return Err("Not enough parameters passed");
		}

		let query = args[1].clone();
		let filename = args[2].clone();

		let case_sensitive = !args.contains(&String::from("--case-insensitive"));
		return Ok(Config { query, filename, case_sensitive });
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one_result() {
		let query = "duct";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

		assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	}

	#[test]
	fn case_insensitive() {
		let query = "rUSt";
		let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(
			vec!["Rust:", "Trust me."],
			search_case_insensitive(query, contents)
		);
	}
}
