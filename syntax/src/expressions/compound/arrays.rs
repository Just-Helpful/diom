use crate::{path::Path, InfoSource};

use super::Expression;

#[derive(InfoSource)]
pub struct Array<I> {
  pub name: Option<Path<I>>,
  pub contents: Vec<Expression<I>>,
  pub info: I,
}
