use super::{SResult, Span};
use crate::tokens::Token;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric1;
use nom::combinator::recognize;
use nom::multi::many0;
use nom::sequence::pair;

pub(crate) fn parse_ident(input: Span) -> SResult<Token> {
  let parse_first = alt((alpha1, tag("_")));
  let parse_rest = alt((alphanumeric1, tag("_")));
  let (input, ident) = recognize(pair(parse_first, many0(parse_rest)))(input)?;
  Ok((input, Token::Ident(ident.into_fragment().into())))
}
