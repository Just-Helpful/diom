use super::{Pattern, Rest};

pub enum ArrayItem<I> {
  Item(Pattern<I>),
  Rest(Rest<I>),
}

pub struct Array<I> {
  pub items: Vec<ArrayItem<I>>,
  pub info: I,
}
