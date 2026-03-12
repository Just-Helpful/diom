use crate::{
  display::{Optn, Sep},
  ident::Ident,
};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{
  collection::vec,
  prelude::{any, Strategy},
};
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
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Struct<I> {
  pub name: Option<Ident<I>>,
  pub fields: Vec<(Ident<I>, Type<I>)>,
  pub info: I,
}

impl<I> Display for Struct<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Optn(&self.name).fmt(f)?;
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
    self.name.write(&mut w.child())?;
    self.fields.write(&mut w.child())?;
    Ok(())
  }
}

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
    (
      any::<Option<Ident<()>>>(),
      vec((any::<Ident<()>>(), item), 0..args.0),
    )
      .prop_map(|(name, fields)| Struct {
        name,
        fields,
        info: (),
      })
  }
}
