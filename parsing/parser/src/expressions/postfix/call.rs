use diom_info_traits::{InfoRef, InfoSource};
use diom_syntax::expressions::{Call, Expression};
use diom_tokens::{SpanTokens, Token};
use nom::multi::separated_list0;

use crate::{
  common::PResult,
  expressions::{parse_expression, postfix::UnaryOperator},
  parsers::token,
  Span,
};

#[derive(InfoSource, InfoRef)]
pub struct CallOp<I> {
  args: Vec<Expression<I>>,
  info: I,
}

impl UnaryOperator for CallOp<Span> {
  fn apply(self, expr: Expression<Self::Info>) -> Expression<Self::Info> {
    Expression::Call(Call {
      info: expr.info().start..self.info.end,
      value: Box::new(expr),
      args: self.args,
    })
  }
}

pub fn parse_call(input: SpanTokens) -> PResult<CallOp<Span>> {
  let (input, lbrac) = token(&Token::LParen)(input)?;
  let (input, args) = separated_list0(token(&Token::Comma), parse_expression)(input)?;
  let (input, rbrac) = token(&Token::RParen)(input)?;

  Ok((
    input,
    CallOp {
      info: lbrac.span.start..rbrac.span.end,
      args,
    },
  ))
}
