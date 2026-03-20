use crate::{
  common::{PResult, Token},
  errors::SyntaxError,
  idents::parse_ident,
  parsers::{group, matches},
  In,
};
use diom_syntax::types::{Function, Parameter, Parameters};
use nom::{
  combinator::{consumed, eof},
  multi::separated_list0,
  sequence::{preceded, terminated},
  Parser,
};

use super::parse_type;

fn parse_parameter<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Parameter<In<'a>>, E> {
  let parser = parse_ident.and(preceded(matches(Token::Colon), parse_type));

  let (input, (info, (name, annotation))) = consumed(parser).parse(input)?;
  Ok((
    input,
    Parameter {
      info,
      name,
      annotation,
    },
  ))
}

fn parse_parameters<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Parameters<In<'a>>, E> {
  let parse_inner = terminated(separated_list0(matches(Token::Comma), parse_parameter), eof);
  let parse_params = group(Token::LParen, Token::RParen).and_then(parse_inner);
  let (input, (info, parameters)) = consumed(parse_params).parse(input)?;
  Ok((input, Parameters { parameters, info }))
}

pub fn parse_function<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Function<In<'a>>, E> {
  let parse_annotation = preceded(matches(Token::Colon), parse_type);
  let parse_function = parse_parameters.and(parse_annotation);

  let (input, (info, (parameters, returned))) = consumed(parse_function).parse(input)?;
  Ok((
    input,
    Function {
      parameters,
      returned: Box::new(returned),
      info,
    },
  ))
}
