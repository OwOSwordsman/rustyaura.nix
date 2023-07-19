pub fn until_attribute_start(input: &str) -> nom::IResult<&str, &str> {
    nom::bytes::complete::take_until("class=\"")(input)
}
