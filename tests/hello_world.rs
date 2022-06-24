use haggis::*;
use std::fs::read_to_string;

const DIRECTORY: &str = "test_data/hello_world/";

macro_rules! gen_test {
	($file:expr) => {
		let contents = read_to_string(format!("{}{}", DIRECTORY, $file)).unwrap();
	run_haggis(contents);
	};
}

// The rest of this would be replaced with a macro if there was a way to get all the files in a directory from the macro
#[test]
fn normal() {
	gen_test!("normal.hgs");
}
#[test]
fn with_comments() {
	gen_test!("with_comments.hgs");
}