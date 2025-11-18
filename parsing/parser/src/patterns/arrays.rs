use super::{parse_pattern, parse_rest};
use crate::{
  Span,
  errors::PResult,
  parsers::{opt_tag_group, token},
  path::parse_path,
};
use diom_syntax::patterns::arrays::{Array, ArrayItem};
use diom_tokens::{SpanTokens, Token};
use nom::{Parser, branch::alt, combinator::eof, multi::separated_list0};

pub fn parse_array_item(input: SpanTokens) -> PResult<ArrayItem<Span>> {
  alt((
    parse_pattern.map(ArrayItem::Item),
    parse_rest.map(ArrayItem::Rest),
  ))(input)
}

pub fn parse_array(input: SpanTokens) -> PResult<Array<Span>> {
  let (input, (name, inner, span)) =
    opt_tag_group(parse_path, Token::LBrace, Token::RBrace)(input)?;
  let (inner, items) = separated_list0(token(Token::Comma), parse_array_item)(inner)?;
  eof(inner)?;

  Ok((
    input,
    Array {
      name,
      items,
      info: span,
    },
  ))
}
