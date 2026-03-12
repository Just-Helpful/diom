use crate::{
  display::{Optn, Sep},
  ident::Ident,
};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{collection::vec, option, prelude::Strategy};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

use super::Type;

/// The type for a combination of indexed fields
///
/// ```_
/// let Vec2 [Float, Float];
/// let Email: [String, String];
///
/// let vec2: Vec2 = Vec2 [1.2, 3.0];
/// let bobs_email: Email = ["bob.jones", "hotmail.com"];
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Tuple<I> {
  pub name: Option<Ident<I>>,
  pub fields: Vec<Type<I>>,
  pub info: I,
}

impl<I> Display for Tuple<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Optn(&self.name).fmt(f)?;
    f.write_char('[')?;
    Sep(&self.fields, ',').fmt(f)?;
    f.write_char(']')
  }
}

impl DisplayAs<Spans> for Tuple<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("tuple", &self.info)?;
    self.name.write(&mut w.child())?;
    self.fields.write(&mut w.child())
  }
}

#[derive(Clone, Copy)]
pub struct TupleConfig(
  /// The maximum number of items in a tuple
  pub usize,
);
impl Default for TupleConfig {
  fn default() -> Self {
    Self(50)
  }
}
impl Tuple<()> {
  /// Generates a generic strategy for generating `Char` types
  pub fn any(
    item: impl Strategy<Value = Type<()>>,
    args: TupleConfig,
  ) -> impl Strategy<Value = Self> {
    (option::of(Ident::any()), vec(item, 0..args.0)).prop_map(|(name, fields)| Tuple {
      name,
      fields,
      info: (),
    })
  }
}
