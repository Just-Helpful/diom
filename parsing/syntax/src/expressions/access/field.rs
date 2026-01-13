use super::Expression;
use crate::{
  fmt::{CustomDisplay, SpanWriter},
  ident::Ident,
};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Field<I> {
  pub value: Box<Expression<I>>,
  pub name: Ident<I>,
  pub info: I,
}

impl CustomDisplay<SpanWriter> for Field<Range<usize>> {
  fn write(&self, w: &mut SpanWriter) -> std::fmt::Result {
    w.bracket("field", &self.info)?;
    self.value.write(&mut w.child())?;
    self.name.write(&mut w.child())
  }
}
