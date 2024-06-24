use crate::InfoSource;

use super::Expression;

#[derive(InfoSource)]
pub struct Index<I> {
  pub value: Box<Expression<I>>,
  pub key: Vec<Expression<I>>,
  pub info: I,
}
