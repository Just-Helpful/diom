use std::ops::Range;

use diom_lexing::{token::Token, tokens::SpanTokens};
use diom_syntax::{
  ident::Ident,
  types::{Enum, Type},
};
use nom::{branch::alt, combinator::opt, multi::separated_list0, Parser};

use crate::{ident::parse_ident, parsers::token, PResult};

use super::{structs::parse_struct, tuples::parse_tuple};

#[allow(clippy::type_complexity)]
fn parse_variant(input: SpanTokens) -> PResult<(Ident<Range<usize>>, Type<Range<usize>>)> {
  // variants *must* start with an identifier
  let (_, name) = parse_ident(input)?;
  let (input, ty) = alt((
    parse_enum.map(Type::Enum),
    parse_struct.map(Type::Struct),
    parse_tuple.map(Type::Tuple),
  ))(input)?;

  Ok((input, (name, ty)))
}

/// Parses a enum-like type.
///
/// For example:
/// ```ignore
/// // named
/// AwaitableNumber {
///   Pending,
///   Error {err: ErrorKind, message: String},
///   Success(Number),
/// }
///
/// // or unnamed
/// {
///   NotConnected,
///   DnsFailure,
///   ServerFailure,
/// }
///
/// // or with no trailing comma
/// { Some(Number), None }
/// ```
pub fn parse_enum(input: SpanTokens) -> PResult<Enum<Range<usize>>> {
  let (input, name) = opt(parse_ident)(input)?;
  let (input, brac) = token(Token::LCurly)(input)?;
  let mut start = brac.span.start;
  if let Some(ref ident) = name {
    start = ident.info.start;
  }

  let (input, variants) = separated_list0(token(Token::Comma), parse_variant)(input)?;

  let (input, brac) = token(Token::RCurly)(input)?;
  let end = brac.span.end;

  Ok((
    input,
    Enum {
      name,
      variants,
      info: start..end,
    },
  ))
}
