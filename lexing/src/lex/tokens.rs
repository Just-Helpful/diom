use nom::{branch::alt, multi::many0, Parser};

use crate::token::SpanToken;

use super::{chars::parse_string, parse_token, token::span_wrap, SResult, Span};

pub fn parse_tokens(input: Span) -> SResult<Vec<SpanToken>> {
  let parse_item = alt((span_wrap(parse_token).map(|tok| vec![tok]), parse_string));
  many0(parse_item)
    .map(|itemss| itemss.into_iter().flatten().collect())
    .parse(input)
}
