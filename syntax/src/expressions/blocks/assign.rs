use super::Expression;
use diom_info::{InfoMap, InfoRef, InfoSource};

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Assign<I> {
  pub name: Ident<I>,
  pub value: Box<Expression<I>>,
  pub info: I,
}
