use super::Expression;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::prelude::Strategy;
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Return<I> {
  pub value: Box<Expression<I>>,
  pub info: I,
}

impl<I> Display for Return<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("return ")?;
    self.value.fmt(f)
  }
}

impl DisplayAs<Spans> for Return<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("return", &self.info)?;
    self.value.write(&mut w.child())
  }
}

impl Return<()> {
  /// Generates a generic strategy for generating `Return` expressions
  pub fn any(item: impl Strategy<Value = Expression<()>>) -> impl Strategy<Value = Self> {
    item.prop_map(|value| Return {
      value: Box::new(value),
      info: (),
    })
  }
}
