use super::Expression;
use crate::{display::Sep, ident::Ident};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Struct<I> {
  pub fields: Vec<(Ident<I>, Expression<I>)>,
  pub info: I,
}

impl<I> Display for Struct<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char('{')?;
    Sep(
      self
        .fields
        .iter()
        .map(|(name, expr)| format!("{name}:{expr}")),
      ',',
    )
    .fmt(f)?;
    f.write_char('}')
  }
}

impl DisplayAs<Spans> for Struct<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("struct", &self.info)?;
    self.fields.write(&mut w.child())
  }
}
