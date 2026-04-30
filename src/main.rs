use std::error::Error;
use katou::parser::parse_single_line_comments;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to Katou - the Lisp Parser");
    Ok(())
}
