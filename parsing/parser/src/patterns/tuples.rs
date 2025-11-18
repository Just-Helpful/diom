use super::{parse_pattern, parse_rest};
use crate::{
  Span,
  errors::PResult,
  parsers::{opt_tag_group, token},
  path::parse_path,
};
use diom_syntax::patterns::tuples::{Tuple, TupleItem};
use diom_tokens::{SpanTokens, Token};
use nom::{Parser, branch::alt, combinator::eof, multi::separated_list0};

pub fn parse_tuple_item(input: SpanTokens) -> PResult<TupleItem<Span>> {
  alt((
    parse_pattern.map(TupleItem::Field),
    parse_rest.map(TupleItem::Rest),
  ))(input)
}

pub fn parse_tuple(input: SpanTokens) -> PResult<Tuple<Span>> {
  let (input, (name, inner, span)) =
    opt_tag_group(parse_path, Token::LParen, Token::RParen)(input)?;
  let (inner, fields) = separated_list0(token(Token::Comma), parse_tuple_item)(inner)?;
  eof(inner)?;

  Ok((
    input,
    Tuple {
      name,
      fields,
      info: span,
    },
  ))
}
