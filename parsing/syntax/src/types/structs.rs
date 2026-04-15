use crate::{display::Sep, idents::Method, Slice};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{collection::vec, prelude::Strategy};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

use super::Type;

/// The type for a combination of named fields.
///
/// ```_
/// let Vec2 {
///   x: Float,
///   y: Float,
/// };
/// let Email: {
///   name: [Char],
///   domain: [Char],
/// };
///
/// let vec2: Vec2 = Vec2 { x: 1.2, y: 3.0 };
/// let bobs_email: Email = { name: "bob.jones", domain: "hotmail.com" };
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug, PartialEq)]
pub struct Struct<I> {
  pub fields: Slice<(Method<I>, Type<I>)>,
  pub info: I,
}

impl<I> Display for Struct<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_char('{')?;
    Sep(
      self.fields.iter().map(|(name, ty)| format!("{name}:{ty}")),
      ',',
    )
    .fmt(f)?;
    f.write_char('}')
  }
}

impl DisplayAs<Spans> for Struct<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("struct", &self.info)?;
    self.fields.write(&mut w.child())?;
    Ok(())
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
  /// Generates a generic strategy for generating `Char` types
  pub fn any(
    item: impl Strategy<Value = Type<()>>,
    args: StructConfig,
  ) -> impl Strategy<Value = Self> {
    vec((Method::any(), item), 0..args.0)
      .prop_map(Slice::from_iter)
      .prop_map(|fields| Struct { fields, info: () })
  }
}
