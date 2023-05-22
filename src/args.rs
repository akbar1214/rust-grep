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
	use std::result;
	use super::*;

	#[test]
	fn invalid_input_less_than_3_arguments() {
		let input: Vec<String> = Vec::new();
		let result = Config::new(&input);
		assert_eq!(result.is_err(), true);
	}
}
