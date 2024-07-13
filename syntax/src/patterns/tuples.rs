use crate::{path::Path, InfoSource};

use super::{Pattern, Rest};

#[derive(InfoSource, Clone)]
pub enum TupleItem<I> {
  Field(Pattern<I>),
  Rest(Rest<I>),
}

#[derive(InfoSource, Clone)]
pub struct Tuple<I> {
  pub name: Option<Path<I>>,
  pub fields: Vec<TupleItem<I>>,
  pub info: I,
}
