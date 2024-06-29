use std::ops::Deref;

use crate::InfoSource;

#[derive(InfoSource, Clone)]
pub struct Ident<I> {
  pub name: Box<str>,
  pub info: I,
}

impl<I> Deref for Ident<I> {
  type Target = str;
  fn deref(&self) -> &Self::Target {
    &self.name
  }
}
