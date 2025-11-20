use super::parse_expression;
use crate::{common::PResult, parsers::token, Span};
use diom_syntax::expressions::Group;
use diom_tokens::{SpanTokens, Token};

pub fn parse_group(input: SpanTokens) -> PResult<Group<Span>> {
  let (input, lbrac) = token(Token::LParen)(input)?;
  let (input, value) = parse_expression(input)?;
  let (input, rbrac) = token(Token::RParen)(input)?;
  Ok((
    input,
    Group {
      value: Box::new(value),
      info: lbrac.span.start..rbrac.span.end,
    },
  ))
}
