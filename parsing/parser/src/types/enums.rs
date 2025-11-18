use crate::{
  Span,
  common::{PResult, SpanTokens, Token},
  parsers::opt_tag_group,
};
use diom_syntax::{
  ident::Ident,
  types::{Enum, Type},
};
use nom::{
  Parser,
  branch::alt,
  combinator::{complete, eof},
  multi::separated_list0,
};

use crate::{ident::parse_ident, parsers::token};

use super::{structs::parse_struct, tuples::parse_tuple};

fn parse_variant(input: SpanTokens) -> PResult<(Ident<Span>, Type<Span>)> {
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
pub fn parse_enum(input: SpanTokens) -> PResult<Enum<Span>> {
  let (input, (name, inner, span)) =
    opt_tag_group(parse_ident, Token::LCurly, Token::RCurly)(input)?;
  let (inner, variants) = complete(separated_list0(token(Token::Comma), parse_variant))(inner)?;
  eof(inner)?;

  Ok((
    input,
    Enum {
      name,
      variants,
      info: span,
    },
  ))
}
