pub fn until_method_start(input: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::take_until(".classes(")(input)
}
