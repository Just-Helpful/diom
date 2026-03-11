use crate::{display::Sep, ident::Ident};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Path<I> {
  pub segments: Vec<Ident<I>>,
  pub info: I,
}

impl<I> Display for Path<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Sep(&self.segments, '.').fmt(f)
  }
}

impl DisplayAs<Spans> for Path<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("path", &self.info)?;
    self.segments.write(&mut w.child())
  }
}
