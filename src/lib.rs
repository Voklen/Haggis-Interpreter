pub fn run_haggis(contents: String) {
	let mut vars = Variables::new();
	for line in contents.lines() {
		if is_comment_or_empty(line) {
			continue;
		}
		let mut words = line.split(' ');
		// .unwrap() will never panic because we've already checked it's not empty
		match words.next().unwrap() {
			"SEND" => send(line, words, &vars),
			"SET" => vars.set(line, words),
			"DECLARE" => vars.declare(line, words),
			command => println!("Unrecognized command: {command}"),
		}
	}
}

/*
INTERNAL FUNCTIONS
*/

fn is_comment_or_empty(line: &str) -> bool {
	let first_char = line.chars().next();
	match first_char {
		Some('#') => true,
		None => true,
		Some(_) => false,
	}
}

/*
EXPRESSION EVALUATIONS
*/

fn evaluate_as_str(input: &str, vars: &Variables) -> String {
	let quote = '"';

	let first_char = input
		.chars()
		.next()
		.expect("Syntax error: Field cannot be empty");
	let last_char = input
		.chars()
		.next_back()
		.expect("Syntax error: Field cannot be empty");

	if (first_char != quote) | (last_char != quote) {
		return vars.get_str(input).clone();
	}

	// At this point all checks have been passed and this is a string surrounded by quotes
	// Remove first and last char (quotes)
	input[1..input.len() - 1].to_string()
}

fn evaluate_as_int(input: &str, vars: &Variables) -> i64 {
	match input.parse::<i64>() {
		Ok(number) => return number,
		Err(_) => {}
	};
	*vars.get_int(input)
}

enum Types {
	String(String),
	Integer(i64),
}

struct Variables {
	vars: std::collections::HashMap<String, Types>,
}
impl Variables {
	fn new() -> Variables {
		use std::collections::HashMap;
		let vars = HashMap::new();
		Variables { vars }
	}
	/// DECLARE found AS BOOLEAN INITIALLY false
	fn declare(&mut self, line: &str, mut words: core::str::Split<char>) {
		let key = words
			.next()
			.expect("Syntax Error: DECLARE must have a variable after")
			.to_string();
		let my_type = match words.next() {
			Some("AS") => Some(words.next().unwrap()),
			Some("INITIALLY") => None,
			_ => panic!("Syntax Error: DECLARE must always have a AS"),
		};

		const INITIALLY_LEN: usize = 11; // " INITIALLY "
		const DECLARE_AS_LEN: usize = 12 + INITIALLY_LEN; // "DECLARE ... AS" + " INITIALLY "
		let var_len = key.len();
		let value = match my_type {
			Some("STRING") => {
				let expr = &line[DECLARE_AS_LEN + var_len + 6..]; // "STRING".len() == 6
				Types::String(evaluate_as_str(expr, self))
			}
			Some("INTEGER") => {
				let expr = &line[DECLARE_AS_LEN + var_len + 7..]; // "INTEGER".len() == 7
				Types::Integer(evaluate_as_int(expr, self))
			}
			Some(invalid_type) => panic!("Unknown type {invalid_type}"),
			None => todo!("Dynamic eval"),
		};

		self.vars.insert(key, value);
	}

	fn set(&mut self, line: &str, mut words: core::str::Split<char>) {
		let key = words.next().unwrap().to_string();
		if words.next() != Some(&"TO") {
			panic!("Syntax Error: SEND must always have a TO")
		}

		let evaluation_section = &line[key.len() + 8..]; // The +8 is the SET ... TO
		let value = match self.vars.get(&key) {
			Some(Types::String(_)) => Types::String(evaluate_as_str(evaluation_section, self)),
			Some(Types::Integer(_)) => Types::Integer(evaluate_as_int(evaluation_section, self)),
			None => panic!("Variable Error: Variable {key} is not declared"),
		};
		self.vars.insert(key, value);
	}
	fn get_str(&self, key: &str) -> &String {
		match self.vars.get(key) {
			Some(Types::String(string)) => string,
			Some(_) => panic!("{key} is not a string"),
			None => panic!("Syntax error: no variable {key}"),
		}
	}
	fn get_int(&self, key: &str) -> &i64 {
		match self.vars.get(key) {
			Some(Types::Integer(int)) => int,
			Some(_) => panic!("{key} is not an integer"),
			None => panic!("Syntax error: no variable {key}"),
		}
	}
}

/*
BUILT IN FUNCTIONS
*/

fn send(line: &str, mut words: core::str::Split<char>, vars: &Variables) {
	let last_word = words.next_back();
	// Check the second to last word is TO
	let second_last_word = words.next_back();
	if second_last_word != Some(&"TO") {
		panic!("Syntax Error: SEND must always have a TO")
	}
	// .unwrap() should never panic as we have just checked that the second to last element exists
	// The +2 is for the spaces between the words
	let expr_end = second_last_word.unwrap().len() + last_word.unwrap().len() + 2;

	let expression = &line[5..(line.len() - expr_end)]; // This gets everything between the SEND and the TO
	let string = evaluate_as_str(expression, vars);
	println!("{string}");
}
