use crate::{
  common::{PResult, Token},
  errors::SyntaxError,
  ident::parse_ident,
  parsers::{group, matches},
  In,
};
use diom_syntax::types::{Argument, Function};
use nom::{
  combinator::{consumed, eof},
  multi::separated_list0,
  sequence::{preceded, terminated},
  Parser,
};

use super::parse_type;

fn parse_argument<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Argument<In<'a>>, E> {
  let parser = parse_ident.and(preceded(matches(Token::Colon), parse_type));

  let (input, (info, (name, annotation))) = consumed(parser).parse(input)?;
  Ok((
    input,
    Argument {
      info,
      name,
      annotation,
    },
  ))
}

pub fn parse_function<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Function<In<'a>>, E> {
  let parse_inner = terminated(separated_list0(matches(Token::Comma), parse_argument), eof);
  let parse_params = group(Token::LParen, Token::RParen).and_then(parse_inner);
  let parse_annotation = preceded(matches(Token::Colon), parse_type);
  let parse_function = parse_params.and(parse_annotation);

  let (input, (info, (arguments, returned))) = consumed(parse_function).parse(input)?;
  Ok((
    input,
    Function {
      arguments,
      returned: Box::new(returned),
      info,
    },
  ))
}
