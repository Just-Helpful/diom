use super::{parse_pattern, parse_rest};
use crate::{
  errors::{PResult, SyntaxError},
  parsers::{group, matches},
  path::parse_path,
  In,
};
use diom_syntax::patterns::tuples::{Tuple, TupleItem};
use diom_tokens::Token;
use nom::{
  branch::alt,
  combinator::{consumed, eof, opt},
  multi::separated_list0,
  sequence::terminated,
  Parser,
};

pub fn parse_tuple_item<'a, E: SyntaxError<'a>>(
  input: In<'a>,
) -> PResult<'a, TupleItem<In<'a>>, E> {
  alt((
    parse_pattern.map(TupleItem::Field),
    parse_rest.map(TupleItem::Rest),
  ))
  .parse(input)
}

pub fn parse_tuple<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Tuple<In<'a>>, E> {
  let parse_inner = terminated(
    separated_list0(matches(Token::Comma), parse_tuple_item),
    eof,
  );
  let parser = opt(parse_path).and(group(Token::LBrace, Token::RBrace).and_then(parse_inner));

  let (input, (info, (name, fields))) = consumed(parser).parse(input)?;
  Ok((input, Tuple { name, fields, info }))
}
