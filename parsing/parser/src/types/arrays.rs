use crate::{
  common::{PResult, Token},
  errors::SyntaxError,
  ident::parse_ident,
  parsers::group,
  In,
};
use diom_syntax::types::Array;
use nom::{
  combinator::{consumed, eof, opt},
  sequence::terminated,
  Parser,
};

use super::parse_type;

pub fn parse_array<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Array<In<'a>>, E> {
  let parse_inner = terminated(parse_type, eof);
  let parser = opt(parse_ident).and(group(Token::LBrace, Token::RBrace).and_then(parse_inner));

  let (input, (info, (name, item))) = consumed(parser).parse(input)?;
  Ok((
    input,
    Array {
      name,
      item: Box::new(item),
      info,
    },
  ))
}
