use crate::{ident::Ident, InfoSource};

use super::{Pattern, Rest};

#[derive(InfoSource)]
pub struct StructField<I> {
  pub name: Ident<I>,
  pub pat: Pattern<I>,
  pub info: I,
}

#[derive(InfoSource)]
pub enum StructItem<I> {
  Field(StructField<I>),
  Rest(Rest<I>),
}

#[derive(InfoSource)]
pub struct Struct<I> {
  pub name: Ident<I>,
  pub fields: Vec<StructField<I>>,
  pub info: I,
}
