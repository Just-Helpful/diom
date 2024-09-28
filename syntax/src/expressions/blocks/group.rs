use super::Expression;
use diom_info::{InfoMap, InfoRef, InfoSource};

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Group<I> {
  pub value: Box<Expression<I>>,
  pub info: I,
}
