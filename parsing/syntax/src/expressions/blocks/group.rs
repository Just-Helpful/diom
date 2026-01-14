use super::Expression;
use diom_fmt::{CustomDisplay, SpanWriter};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Group<I> {
  pub value: Box<Expression<I>>,
  pub info: I,
}

impl CustomDisplay<SpanWriter> for Group<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("group", &self.info)?;
    self.value.write(&mut w.child())
  }
}
