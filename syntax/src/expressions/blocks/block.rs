use crate::InfoSource;

use super::Expression;

#[derive(InfoSource, Clone)]
pub struct Block<I> {
  pub content: Vec<Expression<I>>,
  pub info: I,
}
