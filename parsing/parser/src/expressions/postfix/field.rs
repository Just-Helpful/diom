use diom_info_traits::{InfoRef, InfoSource};
use diom_syntax::{
  expressions::{Expression, Field},
  ident::Ident,
};
use diom_tokens::{SpanTokens, Token};

use crate::{
  common::PResult, expressions::postfix::UnaryOperator, ident::parse_ident, parsers::token, Span,
};

#[derive(InfoSource, InfoRef)]
pub struct FieldOp<I> {
  name: Ident<I>,
  info: I,
}

impl UnaryOperator for FieldOp<Span> {
  fn apply(self, expr: Expression<Self::Info>) -> Expression<Self::Info> {
    Expression::Field(Field {
      info: expr.info().start..self.info.end,
      value: Box::new(expr),
      name: self.name,
    })
  }
}

pub fn parse_field(input: SpanTokens) -> PResult<FieldOp<Span>> {
  let (input, dot) = token(&Token::Dot)(input)?;
  let (input, name) = parse_ident(input)?;
  Ok((
    input,
    FieldOp {
      info: dot.span.start..name.info().end,
      name,
    },
  ))
}
