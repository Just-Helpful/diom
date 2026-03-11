use super::Type;
use crate::{display::Optn, ident::Ident};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

/// A type for arrays of items.
///
/// ```_
/// type String [Char; _];
/// type Nums = [Float];
///
/// let greeting: String = "Hello!";
/// let xs: Nums = [1, 2, 3];
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Array<I> {
  pub name: Option<Ident<I>>,
  pub item: Box<Type<I>>,
  pub info: I,
}

impl<I> Display for Array<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Optn(&self.name).fmt(f)?;
    f.write_char('[')?;
    self.item.fmt(f)?;
    f.write_char(']')
  }
}

impl DisplayAs<Spans> for Array<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("array", &self.info)?;
    self.name.write(&mut w.child())?;
    self.item.write(&mut w.child())
  }
}
