use crate::InfoSource;

use super::Expression;

#[derive(InfoSource)]
pub struct Return<I> {
  pub value: Box<Expression<I>>,
  pub info: I,
}
