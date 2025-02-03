use super::Expression;
use diom_info::{InfoMap, InfoRef, InfoSource};

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Array<I> {
  pub contents: Vec<Expression<I>>,
  pub info: I,
}
