use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};

use crate::parser;

pub fn format_classes(
    surrounding_code: &[&str],
    classes: &[(&str, &str)],
) -> Result<String, Box<dyn Error>> {
    let generated_html = generate_html(classes);
    let formatted_html = run_prettier(&generated_html)?;
    let formatted_classes = parser::parse_html_classes(&formatted_html);

    let mut formatted_code = String::new();
    for (code, start, body) in itertools::izip!(
        surrounding_code.into_iter(),
        classes.into_iter().map(|(start, _)| start),
        formatted_classes.into_iter(),
    ) {
        formatted_code.push_str(code);
        formatted_code.push_str(start);
        formatted_code.push_str(body);
    }
    formatted_code.push_str(surrounding_code.last().unwrap());
    Ok(formatted_code)
}

fn generate_html(classes: &[(&str, &str)]) -> String {
    let mut html = String::new();
    html.push_str("<html>");

    for (_, class_body) in classes {
        html.push_str("<div class=\"");
        html.push_str(class_body);
        html.push_str("\"></div>");
    }

    html.push_str("</html>");
    html
}

fn run_prettier(html: &str) -> Result<String, Box<dyn Error>> {
    let mut prettier = Command::new("prettier")
        .args(["--parser", "html"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    writeln!(prettier.stdin.take().unwrap(), "{}", html)?;

    let output = prettier.wait_with_output()?;
    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err("prettier failed".into())
    }
}
