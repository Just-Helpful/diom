use crate::InfoSource;

use super::Expression;

#[derive(InfoSource)]
pub struct Array<I> {
  pub contents: Vec<Expression<I>>,
  pub info: I,
}
