use crate::display::Sep;

use super::Expression;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{collection::vec, prelude::Strategy};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Index<I> {
  pub value: Box<Expression<I>>,
  pub keys: Vec<Expression<I>>,
  pub info: I,
}

impl<I> Display for Index<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.value.fmt(f)?;
    f.write_char('[')?;
    Sep(&self.keys, ',').fmt(f)?;
    f.write_char(']')
  }
}

impl DisplayAs<Spans> for Index<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("index", &self.info)?;
    self.value.write(&mut w.child())?;
    self.keys.write(&mut w.child())
  }
}

#[derive(Clone, Copy)]
pub struct IndexConfig(
  /// The maximum number of keys in an index expression
  pub usize,
);
impl Default for IndexConfig {
  fn default() -> Self {
    Self(50)
  }
}
impl Index<()> {
  /// Generates a generic strategy for generating `Index` expressions
  pub fn any(
    item: impl Strategy<Value = Expression<()>> + Clone,
    args: IndexConfig,
  ) -> impl Strategy<Value = Self> {
    (item.clone(), vec(item, 0..args.0)).prop_map(|(value, keys)| Index {
      value: Box::new(value),
      keys,
      info: (),
    })
  }
}
