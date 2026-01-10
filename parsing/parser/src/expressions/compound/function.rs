use crate::{
  errors::{PResult, SyntaxError},
  expressions::parse_expression,
  parsers::{group, matches},
  patterns::parse_pattern,
  types::parse_type,
  In,
};

use diom_info_traits::InfoRef;
use diom_syntax::expressions::{Argument, Function, FunctionArm};
use diom_tokens::Token;
use nom::{
  branch::alt,
  combinator::{consumed, eof, opt},
  multi::{many0, separated_list0},
  sequence::{preceded, separated_pair, terminated},
  Parser,
};

/// Parses a function parameter for a given function, i.e.
/// `x: int`, `[x]: [int]`, `{x: name}: {x: number}`
pub fn parse_parameter<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Argument<In<'a>>, E> {
  let parser = parse_pattern.and(opt(preceded(matches(Token::Colon), parse_type)));
  let (input, (info, (pattern, annotation))) = consumed(parser).parse(input)?;
  Ok((
    input,
    Argument {
      pattern,
      annotation,
      info,
    },
  ))
}

pub fn parse_arm<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, FunctionArm<In<'a>>, E> {
  // @todo maybe define type that holds the span for all arguments
  // @todo support `(x)(y) => 3` syntax
  let parse_inner = terminated(many0(parse_parameter), eof);
  let parse_params = group(Token::LParen, Token::RParen).and_then(parse_inner);

  let parse_annotation = opt(preceded(matches(Token::Colon), parse_type));
  let parse_function = separated_pair(
    parse_params.and(parse_annotation), // parameters `(...): ...`
    matches(Token::Function),           // arrow      `=>`
    parse_expression.map(Box::new),     // expression `...`
  );

  let (input, (info, ((arguments, annotation), returned))) =
    consumed(parse_function).parse(input)?;
  Ok((
    input,
    FunctionArm {
      info,
      arguments,
      annotation,
      returned,
    },
  ))
}

pub fn parse_arms<'a, E: SyntaxError<'a>>(
  input: In<'a>,
) -> PResult<'a, (In<'a>, Vec<FunctionArm<In<'a>>>), E> {
  let parse_inner = terminated(separated_list0(matches(Token::Comma), parse_arm), eof);
  let (input, (info, arms)) =
    consumed(group(Token::LCurly, Token::RCurly).and_then(parse_inner)).parse(input)?;

  Ok((input, (info, arms)))
}

pub fn parse_function<'a, E: SyntaxError<'a>>(input: In<'a>) -> PResult<'a, Function<In<'a>>, E> {
  alt((
    parse_arm.map(|arm| Function {
      info: *arm.info(),
      arms: vec![arm],
    }),
    parse_arms.map(|(info, arms_vec)| Function {
      arms: arms_vec,
      info,
    }),
  ))
  .parse(input)
}
