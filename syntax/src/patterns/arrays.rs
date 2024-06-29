use crate::{path::Path, InfoSource};

use super::{Pattern, Rest};

#[derive(InfoSource, Clone)]
pub enum ArrayItem<I> {
  Item(Pattern<I>),
  Rest(Rest<I>),
}

#[derive(InfoSource, Clone)]
pub struct Array<I> {
  pub name: Option<Path<I>>,
  pub items: Vec<ArrayItem<I>>,
  pub info: I,
}
