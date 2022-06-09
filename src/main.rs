fn main() {
	let contents = get_file();
	run_haggis(contents);
}

fn run_haggis(contents: String) {
	let mut vars = Variables::new();
	for line in contents.lines() {
		if is_comment_or_empty(line) {
			continue;
		}
		let words: Vec<&str> = line.split(' ').collect();
		match words[0] {
			"SEND" => send(line, words, &vars),
			"SET" => vars.set(line, words),
			command => println!("Unrecognized command: {command}"),
		}
	}
}

/*
INTERNAL FUNCTIONS
*/

fn get_file() -> String {
	let file_location = "hello_world.hgs";
	std::fs::read_to_string(file_location).expect("Can't read file")
}

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

fn evaluate_as_str<'a>(input: &'a str, vars: &'a Variables) -> String {
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
		return vars.get_str(input);
	}

	// At this point all checks have been passed and this is a string surrounded by quotes
	// Remove first and last char (quotes)
	input[1..input.len() - 1].to_string()
}

enum Types {
	String(String),
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
	fn set(&mut self, line: &str, words: Vec<&str>) {
		if words.get(2) != Some(&"TO") {
			panic!("Syntax Error: SEND must always have a TO")
		}

		let key = words[1].to_string();
		let set_value = evaluate_as_str(&line[key.len() + 8..], self); // The +8 is the SET ... TO
		let value = Types::String(set_value);
		self.vars.insert(key, value);
	}
	fn get_str(&self, key: &str) -> String {
		match self.vars.get(key) {
			Some(Types::String(string)) => string.into(),
			Some(_) => panic!("{key} is not a string"),
			None => panic!("Syntax error: no variable {key}"),
		}
	}
}

/*
BUILT IN FUNCTIONS
*/

fn send(line: &str, words: Vec<&str>, vars: &Variables) {
	// Check the second to last word is TO
	let second_to_last_word = words.get(words.len().wrapping_sub(2));
	if second_to_last_word != Some(&"TO") {
		panic!("Syntax Error: SEND must always have a TO")
	}
	// .unwrap() should never panic as we have just checked that the second to last element exists
	// The +2 is for the spaces between the words
	let expr_end = second_to_last_word.unwrap().len() + words.last().unwrap().len() + 2;

	let expression = &line[5..(line.len() - expr_end)]; // This gets everything between the SEND and the TO
	let string = evaluate_as_str(expression, vars);
	println!("{string}");
}
