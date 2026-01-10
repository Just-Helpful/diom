use super::parse_type;
use crate::{
  common::{PResult, Token},
  errors::SyntaxError,
  ident::parse_ident,
  parsers::{group, matches},
  In,
};
use diom_syntax::types::Struct;
use nom::{
  combinator::{consumed, opt},
  multi::separated_list1,
  sequence::separated_pair,
  Parser,
};

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
pub fn parse_struct<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Struct<In<'a>>, E> {
  let parse_inner = separated_list1(
    matches(Token::Comma),
    separated_pair(parse_ident, matches(Token::Colon), parse_type),
  );
  let parser = opt(parse_ident).and(group(Token::LCurly, Token::RCurly).and_then(parse_inner));

  let (input, (info, (name, fields))) = consumed(parser).parse(input)?;
  Ok((input, Struct { name, fields, info }))
}
