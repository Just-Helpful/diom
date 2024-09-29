use super::Expression;
use diom_info::{InfoMap, InfoRef, InfoSource};

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Call<I> {
  pub value: Box<Expression<I>>,
  pub args: Vec<Expression<I>>,
  pub info: I,
}
