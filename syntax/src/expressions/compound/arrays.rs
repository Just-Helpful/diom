use super::Expression;
use crate::path::Path;
use diom_info::{InfoMap, InfoRef, InfoSource};

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Array<I> {
  pub name: Option<Path<I>>,
  pub contents: Vec<Expression<I>>,
  pub info: I,
}
