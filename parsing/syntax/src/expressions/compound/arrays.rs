use crate::display::Sep;

use super::Expression;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{collection::vec, prelude::Strategy};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Array<I> {
  pub contents: Vec<Expression<I>>,
  pub info: I,
}

impl<I> Display for Array<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char('[')?;
    Sep(&self.contents, ',').fmt(f)?;
    f.write_char(']')
  }
}

impl DisplayAs<Spans> for Array<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("array", &self.info)?;
    self.contents.write(&mut w.child())
  }
}

#[derive(Clone, Copy)]
pub struct ArrayConfig(
  /// The maximum number of items in a array
  pub usize,
);
impl Default for ArrayConfig {
  fn default() -> Self {
    Self(50)
  }
}
impl Array<()> {
  /// Generates a generic strategy for generating `Array` expressions
  pub fn any(
    item: impl Strategy<Value = Expression<()>>,
    args: ArrayConfig,
  ) -> impl Strategy<Value = Self> {
    vec(item, args.0).prop_map(|contents| Array { contents, info: () })
  }
}
