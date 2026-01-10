//! Punctuation specific to creating data structures and separating statements.
use nom::{bytes::complete::tag, IResult, Parser};

pub fn parse_ellipses(input: &str) -> IResult<&str, &str> {
  tag("...").parse(input)
}
pub fn parse_comma(input: &str) -> IResult<&str, &str> {
  tag(",").parse(input)
}
pub fn parse_colon(input: &str) -> IResult<&str, &str> {
  tag(":").parse(input)
}

pub fn parse_semi(input: &str) -> IResult<&str, &str> {
  tag(";").parse(input)
}
