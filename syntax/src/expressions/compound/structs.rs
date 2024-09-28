use super::Expression;
use crate::{ident::Ident, path::Path};
use diom_info::{InfoMap, InfoRef, InfoSource};

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Struct<I> {
  pub name: Option<Path<I>>,
  pub fields: Vec<(Ident<I>, Expression<I>)>,
  pub info: I,
}
