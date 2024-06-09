use std::ops::Range;

use diom_lexing::{token::Token, tokens::SpanTokens};
use diom_syntax::types::Tuple;
use nom::{combinator::opt, multi::separated_list1};

use crate::{ident::parse_ident, parsers::token, PResult};

use super::parse_type;

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
  let (input, name) = opt(parse_ident)(input)?;
  let (input, brac) = token(Token::LParen)(input)?;
  let mut start = brac.span.start;
  if let Some(ref ident) = name {
    start = ident.info.start;
  }

  let (input, fields) = separated_list1(token(Token::Comma), parse_type)(input)?;
  // optional trailing comma
  let (input, _) = opt(token(Token::Comma))(input)?;

  let (input, brac) = token(Token::RParen)(input)?;
  let end = brac.span.end;

  Ok((
    input,
    Tuple {
      name,
      fields,
      info: start..end,
    },
  ))
}
