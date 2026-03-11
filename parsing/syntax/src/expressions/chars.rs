use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Char<I> {
  #[map_ignore]
  pub value: char,
  pub info: I,
}

impl<I> Display for Char<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char('\'')?;
    self.value.fmt(f)?;
    f.write_char('\'')
  }
}

impl DisplayAs<Spans> for Char<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("char", &self.info)
  }
}
