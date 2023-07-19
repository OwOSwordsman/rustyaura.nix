/// returns (surrounding rust code, (tag, css class))
pub fn parse_classes(mut input: &str) -> (Vec<&str>, Vec<(&str, &str)>) {
    let mut surrounding_rust_code = Vec::new();
    let mut css_classes = Vec::new();

    while let Some(
        ClassType::Builder {
            remaining_input,
            rust_code,
            method_call: tag,
        }
        | ClassType::ViewMacro {
            remaining_input,
            rust_code,
            attribute: tag,
        },
    ) = next_class_start(input)
    {
        input = remaining_input;
        surrounding_rust_code.push(rust_code);
        let Ok((remaining_input, css_class)) = class_body(input) else { break };
        css_classes.push((tag, css_class));
        input = remaining_input;
    }

    surrounding_rust_code.push(input);

    (surrounding_rust_code, css_classes)
}

enum ClassType<'a> {
    Builder {
        remaining_input: &'a str,
        rust_code: &'a str,
        method_call: &'a str,
    },
    ViewMacro {
        remaining_input: &'a str,
        rust_code: &'a str,
        attribute: &'a str,
    },
}

fn next_class_start(input: &str) -> Option<ClassType> {
    let next_builder = until_builder_start(input);
    let next_view_macro = until_macro_start(input);

    match (next_builder, next_view_macro) {
        (Err(_), Err(_)) => None,
        (Ok((input, (code, method_call))), Err(_)) => Some(ClassType::Builder {
            remaining_input: input,
            rust_code: code,
            method_call,
        }),
        (Err(_), Ok((input, (code, attribute)))) => Some(ClassType::ViewMacro {
            remaining_input: input,
            rust_code: code,
            attribute,
        }),
        (
            Ok((b_remaining_input, (b_code, method_call))),
            Ok((v_remaining_input, (v_code, attribute))),
        ) => {
            if b_code.len() < v_code.len() {
                Some(ClassType::Builder {
                    remaining_input: b_remaining_input,
                    rust_code: b_code,
                    method_call,
                })
            } else {
                Some(ClassType::ViewMacro {
                    remaining_input: v_remaining_input,
                    rust_code: v_code,
                    attribute,
                })
            }
        }
    }
}

fn class_body(input: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::take_until("\"")(input)
}

/// start of the classes for view! macro
/// returns (remaining code, (rust code, class attribute))
fn until_macro_start(input: &str) -> nom::IResult<&str, (&str, &str)> {
    nom::sequence::tuple((
        nom::bytes::complete::take_until("class=\""),
        nom::bytes::complete::tag("class=\""),
    ))(input)
}

/// start of the classes for builder pattern
/// returns (remaining code, (rust code, method call))
fn until_builder_start(input: &str) -> nom::IResult<&str, (&str, &str)> {
    nom::sequence::tuple((
        nom::bytes::complete::take_until(".classes("),
        nom::bytes::complete::tag(".classes("),
    ))(input)
}

pub fn parse_html_classes(input: &str) -> Vec<&str> {
    parse_classes(input)
        .1
        .into_iter()
        .map(|(_, class_body)| class_body)
        .collect()
}
