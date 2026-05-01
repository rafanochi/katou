use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha0, one_of},
    combinator::{eof, value},
    sequence::preceded,
    IResult, Parser,
};
use std::error::Error;

pub fn symbol(input: &str) -> IResult<&str, char> {
    one_of("!#$%&|*+-/:<=>?@^_~").parse(input)
}

pub fn parse_letters(input: &str) -> IResult<&str, &str> {
    alpha0(input)
}

pub fn parse_bool(input: &str) -> IResult<&str, bool> {
    alt((value(true, tag("true")), value(false, tag("false")))).parse(input)
}

pub fn parse_single_line_comments(input: &str) -> IResult<&str, &str> {
    preceded(tag(";;"), take_while(|x| x != '\n' || x != '\0')).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_line_comment() -> Result<(), Box<dyn Error>> {
        let (remaining, out) =
            parse_single_line_comments(";; Single line comments start with a semicolon")?;
        assert_eq!(out, " Single line comments start with a semicolon");
        assert_eq!(remaining, "");
        Ok(())
    }

    #[test]
    fn check_symbol() -> Result<(), Box<dyn Error>> {
        let (remaining, out) =
            symbol("%abc")?;
        assert_eq!(out, '%');
        Ok(())
    }
}
