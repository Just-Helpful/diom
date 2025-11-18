use super::parse_type;
use crate::{
  common::PResult,
  ident::parse_ident,
  parsers::{opt_tag_group, token},
};
use diom_syntax::types::Tuple;
use diom_tokens::{SpanTokens, Token};
use nom::{
  combinator::{eof, opt},
  multi::separated_list1,
};
use std::ops::Range;

/// Parses a tuple-like type.
///
/// These follow the general form of:
/// ```ignore
/// // named
/// Vec3D (
///   Number,
///   Number,
///   Number,
/// )
///
/// // or unnamed
/// (
///   Operation,
///   Number,
///   Number,
/// )
///
/// // or with no trailing comma
/// ( Number, Number )
/// ```
pub fn parse_tuple(input: SpanTokens) -> PResult<Tuple<Range<usize>>> {
  let (input, (name, inner, span)) =
    opt_tag_group(parse_ident, Token::LParen, Token::RParen)(input)?;
  let (inner, fields) = separated_list1(token(Token::Comma), parse_type)(inner)?;
  let (inner, _) = opt(token(Token::Comma))(inner)?;
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
