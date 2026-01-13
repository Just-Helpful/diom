use super::Type;
use crate::{
  fmt::{CustomDisplay, SpanWriter},
  ident::Ident,
};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

/// A type for arrays of items.
///
/// ```ignore
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

impl CustomDisplay<SpanWriter> for Array<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("array", &self.info)?;
    self.name.write(&mut w.child())?;
    self.item.write(&mut w.child())
  }
}
