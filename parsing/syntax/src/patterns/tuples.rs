use super::{Pattern, Rest};
use crate::path::Path;
use diom_fmt::{CustomDisplay, SpanWriter};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub enum TupleItem<I> {
  Field(Pattern<I>),
  Rest(Rest<I>),
}

impl CustomDisplay<SpanWriter> for TupleItem<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    match self {
      Self::Field(f) => f.write(w),
      Self::Rest(r) => r.write(w),
    }
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Tuple<I> {
  pub name: Option<Path<I>>,
  pub fields: Vec<TupleItem<I>>,
  pub info: I,
}

impl CustomDisplay<SpanWriter> for Tuple<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("tuple", &self.info)?;
    self.name.write(&mut w.child())?;
    self.fields.write(&mut w.child())
  }
}
