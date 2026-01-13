use super::Expression;
use crate::fmt::{CustomDisplay, SpanWriter};
use crate::ident::Ident;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Struct<I> {
  pub fields: Vec<(Ident<I>, Expression<I>)>,
  pub info: I,
}

impl CustomDisplay<SpanWriter> for Struct<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("struct", &self.info)?;
    self.fields.write(&mut w.child())
  }
}
