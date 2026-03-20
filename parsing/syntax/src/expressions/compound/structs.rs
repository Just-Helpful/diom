use super::Expression;
use crate::{display::Sep, idents::Method};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{collection::vec, prelude::Strategy};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Struct<I> {
  pub fields: Vec<(Method<I>, Expression<I>)>,
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

#[derive(Clone, Copy)]
pub struct StructConfig(
  /// The maximum number of fields in a struct
  pub usize,
);
impl Default for StructConfig {
  fn default() -> Self {
    Self(50)
  }
}
impl Struct<()> {
  /// Generates a generic strategy for generating `Struct` expressions
  pub fn any(
    item: impl Strategy<Value = Expression<()>>,
    args: StructConfig,
  ) -> impl Strategy<Value = Self> {
    vec((Method::any(), item), 0..args.0).prop_map(|fields| Struct { fields, info: () })
  }
}
