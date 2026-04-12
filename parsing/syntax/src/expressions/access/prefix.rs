use super::Expression;
use crate::idents::Op;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::prelude::Strategy;
use std::{
  fmt::{Debug, Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Prefix<I> {
  pub name: Op<I>,
  pub value: Box<Expression<I>>,
  pub info: I,
}

impl<I> Display for Prefix<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.name.fmt(f)?;
    f.write_char(' ')?;
    self.value.fmt(f)
  }
}

impl DisplayAs<Spans> for Prefix<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("prefix", &self.info)?;
    self.name.write(&mut w.child())?;
    self.value.write(&mut w.child())
  }
}

impl Prefix<()> {
  /// Generates a generic strategy for generating `Prefix` expressions
  pub fn any(item: impl Strategy<Value = Expression<()>> + Clone) -> impl Strategy<Value = Self> {
    (Op::any(), item).prop_map(|(name, value)| Prefix {
      name,
      value: Box::new(value),
      info: (),
    })
  }
}
