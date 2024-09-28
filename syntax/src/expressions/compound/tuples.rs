use super::Expression;
use crate::path::Path;
use diom_info::{InfoMap, InfoRef, InfoSource};

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Tuple<I> {
  pub name: Option<Path<I>>,
  pub fields: Vec<Expression<I>>,
  pub info: I,
}
