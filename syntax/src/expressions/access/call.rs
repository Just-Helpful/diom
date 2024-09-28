use super::Expression;
use diom_info::{InfoMap, InfoRef, InfoSource};

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Call<I> {
  pub function: Box<Expression<I>>,
  pub arguments: Vec<Expression<I>>,
  pub info: I,
}
