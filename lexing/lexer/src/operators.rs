//! Operators used in expressions and comparisons.
use nom::{bytes::complete::tag, IResult, Parser};

pub fn parse_eq(input: &str) -> IResult<&str, &str> {
  tag("==").parse(input)
}
pub fn parse_ne(input: &str) -> IResult<&str, &str> {
  tag("!=").parse(input)
}
pub fn parse_lt_eq(input: &str) -> IResult<&str, &str> {
  tag("<=").parse(input)
}
pub fn parse_gt_eq(input: &str) -> IResult<&str, &str> {
  tag(">=").parse(input)
}
pub fn parse_lt(input: &str) -> IResult<&str, &str> {
  tag("<").parse(input)
}
pub fn parse_gt(input: &str) -> IResult<&str, &str> {
  tag(">").parse(input)
}
pub fn parse_not(input: &str) -> IResult<&str, &str> {
  tag("!").parse(input)
}
pub fn parse_and(input: &str) -> IResult<&str, &str> {
  tag("&").parse(input)
}
pub fn parse_or(input: &str) -> IResult<&str, &str> {
  tag("|").parse(input)
}
pub fn parse_plus(input: &str) -> IResult<&str, &str> {
  tag("+").parse(input)
}
pub fn parse_minus(input: &str) -> IResult<&str, &str> {
  tag("-").parse(input)
}
pub fn parse_times(input: &str) -> IResult<&str, &str> {
  tag("*").parse(input)
}
pub fn parse_divide(input: &str) -> IResult<&str, &str> {
  tag("/").parse(input)
}
pub fn parse_dot(input: &str) -> IResult<&str, &str> {
  tag(".").parse(input)
}
pub fn parse_assign(input: &str) -> IResult<&str, &str> {
  tag("=").parse(input)
}
