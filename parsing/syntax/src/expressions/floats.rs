use crate::fmt::{CustomDisplay, SpanWriter};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Float<I> {
  #[map_ignore]
  pub value: f64,
  pub info: I,
}

impl CustomDisplay<SpanWriter> for Float<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("float", &self.info)
  }
}
