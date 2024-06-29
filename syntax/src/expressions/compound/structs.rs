use crate::{ident::Ident, path::Path, InfoSource};

use super::Expression;

#[derive(InfoSource, Clone)]
pub struct Struct<I> {
  pub name: Option<Path<I>>,
  pub fields: Vec<(Ident<I>, Expression<I>)>,
  pub info: I,
}
