use super::{SResult, Span, Token};
use nom::{
  branch::alt,
  bytes::complete::{is_not, tag, take_until},
  character::complete::char,
  sequence::{delimited, preceded},
};

fn parse_line(input: Span) -> SResult<Token> {
  let (input, content) = preceded(char('#'), is_not("\n\r"))(input)?;
  Ok((input, Token::Comment(content.into_fragment().into())))
}

fn parse_block(input: Span) -> SResult<Token> {
  let (input, content) = delimited(tag("#("), take_until(")#"), tag(")#"))(input)?;
  Ok((input, Token::Comment(content.into_fragment().into())))
}

pub fn parse_comment(input: Span) -> SResult<Token> {
  alt((parse_line, parse_block))(input)
}
