use crate::{
  common::{PResult, Token},
  errors::SyntaxError,
  parsers::{group, matches},
  types::parse_tagged,
  In,
};
use diom_syntax::types::Enum;
use nom::{
  combinator::{consumed, eof},
  multi::separated_list0,
  sequence::terminated,
  Parser,
};

/// Parses a enum-like type.
///
/// For example:
/// ```_
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
  let parse_inner = terminated(separated_list0(matches(Token::Comma), parse_tagged), eof);
  let parser = group(Token::LCurly, Token::RCurly).and_then(parse_inner);

  let (input, (info, variants)) = consumed(parser).parse(input)?;
  Ok((input, Enum { variants, info }))
}
