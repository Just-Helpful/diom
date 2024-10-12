use super::Expression;
use diom_info::{InfoMap, InfoRef, InfoSource};

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Assign<I> {
  pub reference: Box<Expression<I>>,
  pub value: Box<Expression<I>>,
  pub info: I,
}
