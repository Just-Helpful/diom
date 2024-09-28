use crate::{ident::Ident, path::Path};
use diom_info::{InfoMap, InfoRef, InfoSource};

use super::{Pattern, Rest};

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct StructField<I> {
  pub name: Ident<I>,
  pub pattern: Pattern<I>,
  pub info: I,
}

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub enum StructItem<I> {
  Field(StructField<I>),
  Rest(Rest<I>),
}

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Struct<I> {
  pub name: Option<Path<I>>,
  pub fields: Vec<StructItem<I>>,
  pub info: I,
}
