use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

/// The type for floating point numbers
///
/// ```_
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

impl<I> Display for Float<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("Float")
  }
}

impl DisplayAs<Spans> for Float<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("float", &self.info)
  }
}
