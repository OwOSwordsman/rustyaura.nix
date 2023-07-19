use std::{error::Error, io::Read};

mod parser;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let (surrounding_rust_code, css_classes) = parser::parse_classes(&input);
    dbg!(&surrounding_rust_code);
    dbg!(&css_classes);
    Ok(())
}
