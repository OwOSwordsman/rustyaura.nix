use std::{
    error::Error,
    io::{Read, Write},
    process::{Command, Stdio},
};

mod parser;
mod prettier;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    let formatted_rust = run_rustfmt(&input)?;
    let (surrounding_rust_code, css_classes) = parser::parse_classes(&formatted_rust);
    let formatted_classes = prettier::format_classes(&surrounding_rust_code, &css_classes)?;
    Ok(())
}

fn run_rustfmt(code: &str) -> Result<String, Box<dyn Error>> {
    let mut rustfmt = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    writeln!(rustfmt.stdin.take().unwrap(), "{}", code)?;

    let output = rustfmt.wait_with_output()?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err("rustfmt failed".into())
    }
}
