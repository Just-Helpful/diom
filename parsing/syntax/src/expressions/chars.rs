use crate::fmt::{CustomDisplay, SpanWriter};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Char<I> {
  #[map_ignore]
  pub value: char,
  pub info: I,
}

impl CustomDisplay<SpanWriter> for Char<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("char", &self.info)
  }
}
