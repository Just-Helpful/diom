use super::{parse_pattern, parse_rest};
use crate::{
  errors::{PResult, SyntaxError},
  parsers::{group, matches},
  path::parse_path,
  In,
};
use diom_syntax::patterns::arrays::{Array, ArrayItem};
use diom_tokens::Token;
use nom::{
  branch::alt,
  combinator::{consumed, eof, opt},
  multi::separated_list0,
  sequence::terminated,
  Parser,
};

pub fn parse_array_item<'a, E: SyntaxError<'a>>(
  input: In<'a>,
) -> PResult<'a, ArrayItem<In<'a>>, E> {
  alt((
    parse_pattern.map(ArrayItem::Item),
    parse_rest.map(ArrayItem::Rest),
  ))
  .parse(input)
}

pub fn parse_array<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Array<In<'a>>, E> {
  let parse_inner = terminated(
    separated_list0(matches(Token::Comma), parse_array_item),
    eof,
  );
  let parser = opt(parse_path).and(group(Token::LBrace, Token::RBrace).and_then(parse_inner));

  let (input, (info, (name, items))) = consumed(parser).parse(input)?;
  Ok((input, Array { name, items, info }))
}
