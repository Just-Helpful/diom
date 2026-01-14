use super::Expression;
use diom_fmt::{CustomDisplay, SpanWriter};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{fmt::Debug, ops::Range};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Call<I> {
  pub value: Box<Expression<I>>,
  pub args: Vec<Expression<I>>,
  pub info: I,
}

impl CustomDisplay<SpanWriter> for Call<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("call", &self.info)?;
    self.value.write(&mut w.child())?;
    self.args.write(&mut w.child())
  }
}
