use super::{parse_expression, UnaryOperator};
use crate::{errors::PResult, parsers::token, Span};
use diom_info::{InfoRef, InfoSource};
use diom_syntax::expressions::{Assign, Expression};
use diom_tokens::{SpanTokens, Token};

#[derive(InfoSource, InfoRef)]
pub struct AssignOp<I> {
  value: Box<Expression<I>>,
  info: I,
}

impl UnaryOperator for AssignOp<Span> {
  fn apply(self, expr: Expression<Self::Info>) -> Expression<Self::Info> {
    Expression::Assign(Assign {
      info: expr.info().start..self.info.end,
      reference: Box::new(expr),
      value: self.value,
    })
  }
}

/// Parses an assignment operator
pub fn parse_assign(input: SpanTokens) -> PResult<AssignOp<Span>> {
  let (input, eq) = token(Token::Assign)(input)?;
  let (input, value) = parse_expression(input)?;
  Ok((
    input,
    AssignOp {
      info: eq.span.start..value.info().end,
      value: Box::new(value),
    },
  ))
}
