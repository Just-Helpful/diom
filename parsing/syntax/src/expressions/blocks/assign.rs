use super::Expression;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::prelude::Strategy;
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Assign<I> {
  pub reference: Box<Expression<I>>,
  pub value: Box<Expression<I>>,
  pub info: I,
}

impl<I> Display for Assign<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.reference.fmt(f)?;
    f.write_char('=')?;
    self.value.fmt(f)
  }
}

impl DisplayAs<Spans> for Assign<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("assign", &self.info)?;
    self.reference.write(&mut w.child())?;
    self.value.write(&mut w.child())
  }
}

impl Assign<()> {
  /// Generates a generic strategy for generating `Assign` expressions
  pub fn any(item: impl Strategy<Value = Expression<()>> + Clone) -> impl Strategy<Value = Self> {
    (item.clone(), item).prop_map(|(reference, value)| Assign {
      reference: Box::new(reference),
      value: Box::new(value),
      info: (),
    })
  }
}
