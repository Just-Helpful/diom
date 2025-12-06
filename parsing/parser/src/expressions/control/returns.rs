// use super::Expression;

// pub struct Return<I> {
//   pub value: Box<Expression<I>>,
//   pub info: I,
// }
use super::super::parse_expression;
use crate::{errors::PResult, parsers::token, Span};
use diom_info_traits::InfoRef;
use diom_syntax::expressions::Return;
use diom_tokens::{SpanTokens, Token};

pub fn parse_return(input: SpanTokens) -> PResult<Return<Span>> {
  let (input, tok) = token(Token::Return)(input)?;
  let (input, value) = parse_expression(input)?;
  Ok((
    input,
    Return {
      info: tok.span.start..value.info().end,
      value: Box::new(value),
    },
  ))
}
