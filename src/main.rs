use haggis::*;
pub fn main() {
	let contents = get_file();
	run_haggis(contents);
}

fn get_file() -> String {
	let file_location = "hello_world.hgs";
	std::fs::read_to_string(file_location).expect("Can't read file")
}