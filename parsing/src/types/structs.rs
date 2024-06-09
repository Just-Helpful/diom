use std::ops::Range;

use diom_lexing::{token::Token, tokens::SpanTokens};
use diom_syntax::types::Struct;
use nom::{combinator::opt, multi::separated_list1, sequence::separated_pair};

use crate::{ident::parse_ident, parsers::token, PResult};

use super::parse_type;

/// Parses a struct-like type.
///
/// For example:
/// ```ignore
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
  let (input, name) = opt(parse_ident)(input)?;
  let (input, brac) = token(Token::LCurly)(input)?;
  let mut start = brac.span.start;
  if let Some(ref ident) = name {
    start = ident.info.start;
  }

  let (input, fields) = separated_list1(
    token(Token::Comma),
    separated_pair(parse_ident, token(Token::Colon), parse_type),
  )(input)?;
  let (input, _) = opt(token(Token::Comma))(input)?;

  let (input, brac) = token(Token::RCurly)(input)?;
  let end = brac.span.end;

  Ok((
    input,
    Struct {
      name,
      fields,
      info: start..end,
    },
  ))
}
