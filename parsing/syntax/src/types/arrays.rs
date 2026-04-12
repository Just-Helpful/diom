use super::Type;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::prelude::Strategy;
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
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Array<I> {
  pub item: Box<Type<I>>,
  pub info: I,
}

impl<I> Display for Array<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char('[')?;
    self.item.fmt(f)?;
    f.write_char(']')
  }
}

impl DisplayAs<Spans> for Array<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("array", &self.info)?;
    self.item.write(&mut w.child())
  }
}

impl Array<()> {
  /// Generates a generic strategy for generating `Array` types
  pub fn any(item: impl Strategy<Value = Type<()>>) -> impl Strategy<Value = Self> {
    item.prop_map(|item| Array {
      item: Box::new(item),
      info: (),
    })
  }
}
