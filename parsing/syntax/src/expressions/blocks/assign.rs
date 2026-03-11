use super::Expression;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
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
