use crate::{
  common::{PResult, Token},
  errors::SyntaxError,
  parsers::group,
  In,
};
use diom_syntax::{types::Array, Ptr};
use nom::{
  combinator::{consumed, eof},
  sequence::terminated,
  Parser,
};

use super::parse_type;

pub fn parse_array<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Array<In<'a>>, E> {
  let parse_inner = terminated(parse_type, eof);
  let parser = group(Token::LBrace, Token::RBrace).and_then(parse_inner);

  let (input, (info, item)) = consumed(parser).parse(input)?;
  Ok((
    input,
    Array {
      item: Ptr::new(item),
      info,
    },
  ))
}
