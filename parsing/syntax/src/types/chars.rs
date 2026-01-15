use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{fmt::Write, ops::Range};

/// The type for single characters
///
/// ```ignore
/// let SingleString: Char;
/// let c: Char = 'v';
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Char<I> {
  pub info: I,
}

impl DisplayAs<Spans> for Char<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("char", &self.info)
  }
}
