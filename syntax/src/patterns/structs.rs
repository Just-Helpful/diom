use crate::{ident::Ident, path::Path, InfoSource};

use super::{Pattern, Rest};

#[derive(InfoSource, Clone)]
pub struct StructField<I> {
  pub name: Ident<I>,
  pub pattern: Pattern<I>,
  pub info: I,
}

#[derive(InfoSource, Clone)]
pub enum StructItem<I> {
  Field(StructField<I>),
  Rest(Rest<I>),
}

#[derive(InfoSource, Clone)]
pub struct Struct<I> {
  pub name: Option<Path<I>>,
  pub fields: Vec<StructItem<I>>,
  pub info: I,
}
