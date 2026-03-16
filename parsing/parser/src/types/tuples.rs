use super::parse_type;
use crate::{
  common::PResult,
  errors::SyntaxError,
  parsers::{group, matches},
  In,
};
use diom_syntax::types::Tuple;
use diom_tokens::Token;
use nom::{
  combinator::{consumed, eof},
  multi::separated_list1,
  sequence::terminated,
  Parser,
};

/// Parses a tuple-like type.
///
/// These follow the general form of:
/// ```_
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
pub fn parse_tuple<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Tuple<In<'a>>, E> {
  let parse_inner = terminated(separated_list1(matches(Token::Comma), parse_type), eof);
  let parser = group(Token::LBrace, Token::RBrace).and_then(parse_inner);

  let (input, (info, fields)) = consumed(parser).parse(input)?;
  Ok((input, Tuple { fields, info }))
}
