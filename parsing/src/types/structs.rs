use super::parse_type;
use crate::{
  common::{PResult, SpanTokens, Token},
  ident::parse_ident,
  parsers::{opt_tag_group, token},
};
use diom_syntax::types::Struct;
use nom::{combinator::eof, multi::separated_list1, sequence::separated_pair};
use std::ops::Range;

/// Parses a struct-like type.
///
/// For example:
/// ```_,ignore
/// // named
/// Vec2D {
///   x: Number,
///   y: Number,
/// }
///
/// // or unnamed
/// {
///   id: Number,
///   name: String,
/// }
///
/// // or in a more compact format
/// { id: Number, name: String }
/// ```
pub fn parse_struct(input: SpanTokens) -> PResult<Struct<Range<usize>>> {
  let (input, (name, inner, span)) =
    opt_tag_group(parse_ident, Token::LCurly, Token::RCurly)(input)?;
  let (inner, fields) = separated_list1(
    token(Token::Comma),
    separated_pair(parse_ident, token(Token::Colon), parse_type),
  )(inner)?;
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
