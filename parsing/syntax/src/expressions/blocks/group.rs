use super::Expression;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::prelude::Strategy;
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Group<I> {
  pub value: Box<Expression<I>>,
  pub info: I,
}

impl<I> Display for Group<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char('(')?;
    self.value.fmt(f)?;
    f.write_char(')')
  }
}

impl DisplayAs<Spans> for Group<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("group", &self.info)?;
    self.value.write(&mut w.child())
  }
}

impl Group<()> {
  /// Generates a generic strategy for generating `Group` expressions
  pub fn any(item: impl Strategy<Value = Expression<()>>) -> impl Strategy<Value = Self> {
    item.prop_map(|value| Group {
      value: Box::new(value),
      info: (),
    })
  }
}
