use crate::{
  common::{PResult, Token},
  errors::SyntaxError,
  parsers::group,
  In,
};
use diom_syntax::{
  ident::Ident,
  types::{Enum, Type},
};
use nom::{
  branch::alt,
  combinator::{consumed, eof, opt},
  multi::separated_list0,
  sequence::terminated,
  Parser,
};

use crate::{ident::parse_ident, parsers::matches};

use super::{structs::parse_struct, tuples::parse_tuple};

fn parse_variant<'a, E: SyntaxError<'a>>(
  input: In<'a>,
) -> PResult<'a, (Ident<In<'a>>, Type<In<'a>>), E> {
  // variants *must* start with an identifier
  let (_, name) = parse_ident(input)?;
  let (input, ty) = alt((
    parse_enum.map(Type::Enum),
    parse_struct.map(Type::Struct),
    parse_tuple.map(Type::Tuple),
  ))
  .parse(input)?;

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
pub fn parse_enum<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Enum<In<'a>>, E> {
  let parse_inner = terminated(separated_list0(matches(Token::Comma), parse_variant), eof);
  let parser = opt(parse_ident).and(group(Token::LCurly, Token::RCurly).and_then(parse_inner));

  let (input, (info, (name, variants))) = consumed(parser).parse(input)?;
  Ok((
    input,
    Enum {
      name,
      variants,
      info,
    },
  ))
}
