use crate::display::Sep;

use super::Expression;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{
  fmt::{Debug, Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Call<I> {
  pub value: Box<Expression<I>>,
  pub args: Vec<Expression<I>>,
  pub info: I,
}

impl<I> Display for Call<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.value.fmt(f)?;
    f.write_char('(')?;
    Sep(&self.args, ',').fmt(f)?;
    f.write_char(')')
  }
}

impl DisplayAs<Spans> for Call<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("call", &self.info)?;
    self.value.write(&mut w.child())?;
    self.args.write(&mut w.child())
  }
}
