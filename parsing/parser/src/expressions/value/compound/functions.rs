use crate::{
  Span,
  errors::PResult,
  expressions::parse_expression,
  parsers::{group, token},
  patterns::parse_pattern,
  types::parse_type,
};

use diom_info_traits::InfoRef;
use diom_syntax::expressions::{Argument, Function, FunctionArm};
use diom_tokens::{SpanTokens, Token};
use nom::{
  Parser,
  branch::alt,
  combinator::{eof, opt},
  multi::{many0, separated_list0},
  sequence::preceded,
};

pub fn parse_argument(input: SpanTokens) -> PResult<Argument<Span>> {
  let (input, pattern) = parse_pattern(input)?;
  let (input, annotation) = opt(preceded(token(Token::Colon), parse_type))(input)?;
  let mut info = pattern.info().clone();
  if let Some(span) = annotation.as_ref().map(|ann| ann.info()) {
    info.end = span.end;
  }
  Ok((
    input,
    Argument {
      pattern,
      annotation,
      info,
    },
  ))
}

pub fn parse_function_arm(input: SpanTokens) -> PResult<FunctionArm<Span>> {
  // @todo maybe define type that holds the span for all arguments
  // @todo support `(x)(y) => 3` syntax
  let (input, (inner, span)) = group(Token::LParen, Token::RParen)(input)?;
  let (inner, arguments) = many0(parse_argument)(inner)?;
  eof(inner)?;

  let (input, annotation) = opt(preceded(token(Token::Colon), parse_type))(input)?;
  let (input, _) = token(Token::Assign)(input)?;
  let (input, returned) = parse_expression.map(Box::new).parse(input)?;
  Ok((
    input,
    FunctionArm {
      info: span.start..returned.info().end,
      arguments,
      annotation,
      returned,
    },
  ))
}

pub fn parse_function_arms(input: SpanTokens) -> PResult<(Span, Vec<FunctionArm<Span>>)> {
  let (input, (inner, span)) = group(Token::LParen, Token::RParen)(input)?;
  let (inner, arms) = separated_list0(token(Token::Comma), parse_function_arm)(inner)?;
  eof(inner)?;
  Ok((input, (span, arms)))
}

pub fn parse_function(input: SpanTokens) -> PResult<Function<Span>> {
  alt((
    parse_function_arm.map(|arm| Function {
      info: arm.info().clone(),
      arms: vec![arm],
    }),
    parse_function_arms.map(|(span, arms_vec)| Function {
      info: span,
      arms: arms_vec,
    }),
  ))(input)
}
