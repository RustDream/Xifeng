extern crate xifeng;

use xifeng::Document;

fn main() {
    println!("Done");
    let mut file: Document = Document::new("test.md");
    match file.read() {
        Err(e) => println!("{}", e),
        _ => {},
    }
    println!("{}", file.name());
}