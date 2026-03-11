use super::Expression;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
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
