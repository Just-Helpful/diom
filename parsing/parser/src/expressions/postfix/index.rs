use diom_info_traits::{InfoRef, InfoSource};
use diom_syntax::expressions::{Expression, Index};
use diom_tokens::{SpanTokens, Token};
use nom::multi::separated_list1;

use crate::{
  common::{PResult, Span},
  expressions::{UnaryOperator, parse_expression},
  parsers::token,
};

#[derive(InfoSource, InfoRef)]
pub struct IndexOp<I> {
  key: Vec<Expression<I>>,
  info: I,
}

impl UnaryOperator for IndexOp<Span> {
  fn apply(self, expr: Expression<Self::Info>) -> Expression<Self::Info> {
    Expression::Index(Index {
      info: expr.info().start..self.info.end,
      value: Box::new(expr),
      key: self.key,
    })
  }
}

pub fn parse_index(input: SpanTokens) -> PResult<IndexOp<Span>> {
  let (input, lbrac) = token(&Token::LBrace)(input)?;
  let (input, key) = separated_list1(token(Token::Comma), parse_expression)(input)?;
  let (input, rbrac) = token(&Token::RBrace)(input)?;

  Ok((
    input,
    IndexOp {
      info: lbrac.span.start..rbrac.span.end,
      key,
    },
  ))
}
