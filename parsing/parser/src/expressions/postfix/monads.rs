use crate::{Span, errors::PResult, parsers::token};
use diom_info_traits::{InfoRef, InfoSource};
use diom_syntax::expressions::{Expression, MonadThen};
use diom_tokens::{SpanTokens, Token};

use super::UnaryOperator;

#[derive(InfoSource, InfoRef)]
pub struct MonadOp<I> {
  info: I,
}

impl UnaryOperator for MonadOp<Span> {
  fn apply(self, expr: Expression<Self::Info>) -> Expression<Self::Info> {
    Expression::Monad(MonadThen {
      info: expr.info().start..self.info.end,
      value: Box::new(expr),
    })
  }
}

/// Parses a monadic application, of the form `x?`
pub fn parse_monad(input: SpanTokens) -> PResult<MonadOp<Span>> {
  let (input, monad) = token(&Token::Monad)(input)?;
  Ok((input, MonadOp { info: monad.span }))
}
