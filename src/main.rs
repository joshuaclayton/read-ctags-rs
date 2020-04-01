use read_ctags_rs::{ctag_item::CtagItem, read_file};
use serde_json;

fn main() {
    let filenames = vec![".git/tags", "tags", "tmp/tags"];

    match read_file::read_first_available_to_string(&filenames) {
        Ok(contents) => match CtagItem::parse(&contents) {
            Ok(("", outcome)) => println!("{}", serde_json::to_string(&outcome).unwrap()),
            _ => eprintln!("Unable to fully parse file"),
        },
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
