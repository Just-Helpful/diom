use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

/// The type for single characters
///
/// ```_
/// let SingleString: Char;
/// let c: Char = 'v';
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Char<I> {
  pub info: I,
}

impl<I> Display for Char<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("Char")
  }
}

impl DisplayAs<Spans> for Char<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("char", &self.info)
  }
}
