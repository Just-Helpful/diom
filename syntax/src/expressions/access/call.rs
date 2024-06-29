use crate::InfoSource;

use super::Expression;

#[derive(InfoSource, Clone)]
pub struct Call<I> {
  pub function: Box<Expression<I>>,
  pub arguments: Vec<Expression<I>>,
  pub info: I,
}
