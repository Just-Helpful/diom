use crate::{
  errors::PResult,
  expressions::parse_expression,
  ident::parse_ident,
  parsers::{opt_tag_group, token},
  path::parse_path,
  Span,
};
use diom_syntax::{
  expressions::{Expression, Struct},
  ident::Ident,
};
use diom_tokens::{SpanTokens, Token};
use nom::{combinator::eof, multi::separated_list0};

/// Parses a single struct field
/// @todo maybe expand this to allow for `Foo { x }` like declerations
/// where `x` is a variable defined before `Foo`
pub fn parse_struct_field(input: SpanTokens) -> PResult<(Ident<Span>, Expression<Span>)> {
  let (input, field) = parse_ident(input)?;
  let (input, _) = token(Token::Colon)(input)?;
  let (input, value) = parse_expression(input)?;
  Ok((input, (field, value)))
}

pub fn parse_struct(input: SpanTokens) -> PResult<Struct<Span>> {
  let (input, (name, inner, span)) =
    opt_tag_group(parse_path, Token::LCurly, Token::RCurly)(input)?;
  let (inner, fields) = separated_list0(token(Token::Comma), parse_struct_field)(inner)?;
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
