use nom::branch::permutation;
use nom::character::complete::digit1;
use nom::combinator::{map_res, opt};
use nom::bytes::complete::tag_no_case;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub language: String
}

/// parse age
fn age_parser(input: &str) -> IResult<&str, u8> {
    let (input, _) = opt(tag_no_case(" and"))(input)?;
    let (input, _) = tag_no_case(" I am ")(input)?;
    let (input, age) = map_res(digit1,|x: &str| x.parse::<u8>())(input)?;
    let (input, _) = tag_no_case(" years old")(input)?;
    Ok((input, age))
}

/// parse language
fn language_parser(input: &str) -> IResult<&str, String> {
    let (input, _) = opt(tag_no_case(" and"))(input)?;
    let (input, _) = tag_no_case(" I like ")(input)?;
    let (input, language) = alpha1(input)?;
    Ok((input, language.to_string()))
}


/// parse name 
fn name_parser(input: &str) -> IResult<&str, String> {
  let (input, _) = tag("my name is ")(input)?;
  let (input, name) = alpha1(input)?;
  Ok((input, name.to_string())) 
}


/// parse person
fn parse_person<'a>(input: &str) -> IResult<&str, Person> {
    // "Hello my name is Tomaso and I am 32 years old and I like Rust"
    // "Hello my name is Sunkanmi and I like Python and I am 44 years old"
    let (input, _) = tag("Hello, ")(input)?;
    let (input, (name, age, language)) = permutation((name_parser, age_parser, language_parser))(input)?;
    Ok((input, Person{name, age, language}))
}


fn main() {
  dbg!(parse_person("Hello, my name is Tomaso and I am 32 years old and I like Rust"));
  dbg!(parse_person("Hello, my name is DB and I am 45 years old and I like Golang"));
  dbg!(parse_person("Hello, my name is Sunkanmi and I like Python and I am 44 years old"));
  dbg!(parse_person("Hello, my name is Yomade and I like Rust and I am 44 years old"));
}
