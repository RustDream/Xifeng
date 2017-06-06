extern crate xifeng;

use xifeng::Document;

fn main() {
	{
		let mut file: Document = Document::new("README.md");
		let mut output: Document = Document::new("test.md");
		file.read();
		output.write(file.date());
		output.save();
	}
	let mut test: Document = Document::new("test.md");
	match test.read() {
        Err(e) => println!("{}", e),
        _ => {},
    }
    println!("{}", test.name());
	println!("--------------------");
	println!("{}", test.date_as_string());
	test.kill();
}