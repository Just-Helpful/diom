use super::{Pattern, Rest};
use crate::path::Path;
use diom_fmt::{CustomDisplay, SpanWriter};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub enum ArrayItem<I> {
  Item(Pattern<I>),
  Rest(Rest<I>),
}

impl CustomDisplay<SpanWriter> for ArrayItem<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    match self {
      Self::Item(i) => i.write(w),
      Self::Rest(r) => r.write(w),
    }
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Array<I> {
  pub name: Option<Path<I>>,
  pub items: Vec<ArrayItem<I>>,
  pub info: I,
}

impl CustomDisplay<SpanWriter> for Array<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("array", &self.info)?;
    self.name.write(&mut w.child())?;
    self.items.write(&mut w.child())
  }
}
