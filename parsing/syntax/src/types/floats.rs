use diom_fmt::{CustomDisplay, SpanWriter};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

/// The type for floating point numbers
///
/// ```ignore
/// let Number: Float;
/// let x: Float = 1;
/// let x: Float = 1.0;
/// let x: Float = 1.0e1;
/// let x: Float = -1e-1;
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Float<I> {
  pub info: I,
}

impl CustomDisplay<SpanWriter> for Float<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("float", &self.info)
  }
}
