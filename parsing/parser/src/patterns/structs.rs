use super::{parse_pattern, parse_rest};
use crate::{
  Span,
  errors::PResult,
  ident::parse_ident,
  parsers::{opt_tag_group, token},
  path::parse_path,
};
use diom_info_traits::InfoRef;
use diom_syntax::patterns::{
  Pattern,
  structs::{Struct, StructField, StructItem},
};
use diom_tokens::{SpanTokens, Token};
use nom::{
  Parser,
  branch::alt,
  combinator::{eof, opt},
  multi::separated_list1,
  sequence::preceded,
};

pub fn parse_struct_field(input: SpanTokens) -> PResult<StructField<Span>> {
  let (input, name) = parse_ident(input)?;
  let (input, pattern) = opt(preceded(token(Token::Colon), parse_pattern))(input)?;
  let pattern = pattern.unwrap_or_else(|| Pattern::Var(name.clone()));
  let info = name.info().start..pattern.info().end;
  Ok((
    input,
    StructField {
      name,
      pattern,
      info,
    },
  ))
}

pub fn parse_struct_item(input: SpanTokens) -> PResult<StructItem<Span>> {
  alt((
    parse_struct_field.map(StructItem::Field),
    parse_rest.map(StructItem::Rest),
  ))(input)
}

pub fn parse_struct(input: SpanTokens) -> PResult<Struct<Span>> {
  let (input, (name, inner, span)) =
    opt_tag_group(parse_path, Token::LCurly, Token::RCurly)(input)?;
  let (inner, fields) = separated_list1(token(Token::Comma), parse_struct_item)(inner)?;
  eof(inner)?;

  Ok((
    input,
    Struct {
      name,
      fields,
      info: span,
    },
  ))
}
