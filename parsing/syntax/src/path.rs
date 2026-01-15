use crate::ident::Ident;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{fmt::Write, ops::Range};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Path<I> {
  pub segments: Vec<Ident<I>>,
  pub info: I,
}

impl DisplayAs<Spans> for Path<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("path", &self.info)?;
    self.segments.write(&mut w.child())
  }
}
