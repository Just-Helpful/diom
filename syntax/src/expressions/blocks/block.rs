use crate::InfoSource;

use super::Expression;

#[derive(InfoSource)]
pub struct Block<I> {
  pub content: Vec<Expression<I>>,
  pub info: I,
}
