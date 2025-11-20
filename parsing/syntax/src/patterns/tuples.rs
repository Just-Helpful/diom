use crate::path::Path;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};

use super::{Pattern, Rest};

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub enum TupleItem<I> {
  Field(Pattern<I>),
  Rest(Rest<I>),
}

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Tuple<I> {
  pub name: Option<Path<I>>,
  pub fields: Vec<TupleItem<I>>,
  pub info: I,
}
