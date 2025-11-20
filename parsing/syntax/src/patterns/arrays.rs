use crate::path::Path;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};

use super::{Pattern, Rest};

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub enum ArrayItem<I> {
  Item(Pattern<I>),
  Rest(Rest<I>),
}

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Array<I> {
  pub name: Option<Path<I>>,
  pub items: Vec<ArrayItem<I>>,
  pub info: I,
}
