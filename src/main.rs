fn main() {
	let contents = get_file();
	run_haggis(contents);
}

fn run_haggis(contents: String) {
	for i in contents.lines() {
		// Check if comment
		let first_char = i.chars().next();
		match first_char {
			Some('#') => {println!("comment");continue}
			None => continue,
			Some(_) => {}
		}

		// Run line
		let words: Vec<&str> = i.split(' ').collect();
		match words[0] {
			"SEND" => send(i, words),
			_ => println!("Unrecognized command: {}", words[0]),
		}
	}
}

fn get_file() -> String {
	let file_location = "hello_world.hggs";
	std::fs::read_to_string(file_location)
		.expect("Can't read file")
}

/*
EXPRESSION EVALUATIONS
*/

fn evaluate_as_str(input: &str) -> &str {
	let quote = '"';

	let first_char = input
		.chars().next() // Get first char
		.expect("Syntax error: Field cannot be empty");
	let last_char = input
		.chars().next_back() // Get last char
		.expect("Syntax error: Field cannot be empty");

	if (first_char != quote) | (last_char != quote) {
		return get_var_as_str(input);
	}

	// At this point all checks have been passed and this is a string surrounded by quotes
	// Remove first and last char (quotes)
	let mut chars = input.chars();
	chars.next();
	chars.next_back();
	chars.as_str()
}

/*
VARIABLE GETTERS
*/

fn get_var_as_str(input: &str) -> &str {
	panic!("Syntax error: no variable {}", input);
}

/*
BUILT IN FUNCTIONS
*/

fn send(to_send: &str, words: Vec<&str>) {
	let second_to_last_word = words.get(words.len().wrapping_sub(2));
	if second_to_last_word != Some(&"TO") {
		panic!("Syntax Error: SEND must always have a TO")
	}
	let expr_end = second_to_last_word.unwrap().len() + words.last().unwrap().len() + 2; // .unwrap() should never panic as we have just checked that the second to last element exists (The +2 is for the spaces between the words)

	let expression = &to_send[5..(to_send.len() - expr_end)]; // This get's everything between the SEND and the TO
	let string = evaluate_as_str(expression);
	println!("{}", string);
}
