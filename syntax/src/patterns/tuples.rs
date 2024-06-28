use crate::{ident::Ident, InfoSource};

use super::{Pattern, Rest};

#[derive(InfoSource)]
pub enum TupleField<I> {
  Field(Pattern<I>),
  Rest(Rest<I>),
}

#[derive(InfoSource)]
pub struct Tuple<I> {
  pub name: Ident<I>,
  pub fields: Vec<TupleField<I>>,
  pub info: I,
}
