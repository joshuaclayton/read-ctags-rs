use std::fs;
use std::io;
use std::io::{Error, ErrorKind};

pub fn read_first_available_to_string(filenames: &[&str]) -> Result<String, io::Error> {
    return first_success(
        filenames,
        Error::new(ErrorKind::Other, "No file provided"),
        run,
    );
}

fn first_success<A, B, C, F>(values: &[A], default: C, f: F) -> Result<B, C>
where
    A: Copy,
    F: Fn(A) -> Result<B, C>,
{
    let mut outcome = Err(default);
    for &x in values.iter() {
        outcome = f(x);
        if outcome.is_ok() {
            break;
        }
    }
    outcome
}

fn run(filename: &str) -> Result<String, io::Error> {
    let contents = fs::read_to_string(filename)?;

    Ok(contents)
}
