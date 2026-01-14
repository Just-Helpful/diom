use super::{Pattern, Rest};
use crate::{ident::Ident, path::Path};
use diom_fmt::{CustomDisplay, SpanWriter};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct StructField<I> {
  pub name: Ident<I>,
  pub pattern: Pattern<I>,
  pub info: I,
}

impl CustomDisplay<SpanWriter> for StructField<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("field", &self.info)?;
    self.name.write(&mut w.child())?;
    self.pattern.write(&mut w.child())
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub enum StructItem<I> {
  Field(StructField<I>),
  Rest(Rest<I>),
}

impl CustomDisplay<SpanWriter> for StructItem<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    match self {
      Self::Field(f) => f.write(w),
      Self::Rest(r) => r.write(w),
    }
  }
}

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Struct<I> {
  pub name: Option<Path<I>>,
  pub fields: Vec<StructItem<I>>,
  pub info: I,
}

impl CustomDisplay<SpanWriter> for Struct<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("struct", &self.info)?;
    self.name.write(&mut w.child())?;
    self.fields.write(&mut w.child())
  }
}
