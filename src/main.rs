mod ctag_parser;
mod language;
mod read_file;
mod token_kind;
use serde_json;

fn main() {
    let filenames = vec![".git/tags", "tags", "tmp/tags"];

    match read_file::read_first_available_to_string(&filenames) {
        Ok(contents) => match ctag_parser::parse(&contents) {
            Ok(("", outcome)) => println!("{}", serde_json::to_string(&outcome).unwrap()),
            v => println!("{:?}", v),
        },
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
