use super::Expression;
use crate::fmt::{CustomDisplay, SpanWriter};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Array<I> {
  pub contents: Vec<Expression<I>>,
  pub info: I,
}

impl CustomDisplay<SpanWriter> for Array<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("array", &self.info)?;
    self.contents.write(&mut w.child())
  }
}
