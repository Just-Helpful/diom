use super::Expression;
use crate::fmt::{CustomDisplay, SpanWriter};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Index<I> {
  pub value: Box<Expression<I>>,
  pub key: Vec<Expression<I>>,
  pub info: I,
}

impl CustomDisplay<SpanWriter> for Index<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("index", &self.info)?;
    self.value.write(&mut w.child())?;
    self.key.write(&mut w.child())
  }
}
