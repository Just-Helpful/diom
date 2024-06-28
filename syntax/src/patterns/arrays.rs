use crate::{ident::Ident, InfoSource};

use super::{Pattern, Rest};

#[derive(InfoSource)]
pub enum ArrayItem<I> {
  Item(Pattern<I>),
  Rest(Rest<I>),
}

#[derive(InfoSource)]
pub struct Array<I> {
  pub name: Option<Ident<I>>,
  pub items: Vec<ArrayItem<I>>,
  pub info: I,
}
