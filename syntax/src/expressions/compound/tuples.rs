use crate::{path::Path, InfoSource};

use super::Expression;

#[derive(InfoSource, Clone)]
pub struct Tuple<I> {
  pub name: Option<Path<I>>,
  pub fields: Vec<Expression<I>>,
  pub info: I,
}
