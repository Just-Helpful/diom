use crate::{common::PResult, expressions::value::parse_value, ident::parse_ident, Span};
use diom_syntax::{expressions::Expression, ident::Ident};
use diom_tokens::SpanTokens;

pub fn parse_infix(input: SpanTokens) -> PResult<(Ident<Span>, Expression<Span>)> {
  let (input, name) = parse_ident(input)?;
  let (input, other) = parse_value(input)?;
  Ok((input, (name, other)))
}
