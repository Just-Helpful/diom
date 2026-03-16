use super::Type;
use crate::{
  ident::Ident,
  types::{Tagged, TypeConfig},
};
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use proptest::{prelude::Strategy, prop_oneof};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

/// A type definition that aliases a name to a type
///
/// ```ignore
/// type Number = Float;
/// type Boolean = Bool;
/// type Vec2_0 = {x: Float, y: Float};
/// type Vec3_0 = [Float, Float, Float];
/// type VecN_0 = [Float; _];
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Alias<I> {
  pub name: Ident<I>,
  pub value: Box<Type<I>>,
  pub info: I,
}

impl<I> Display for Alias<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("type ")?;
    self.name.fmt(f)?;
    f.write_str(" = ")?;
    self.value.fmt(f)
  }
}

impl DisplayAs<Spans> for Alias<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("type alias", &self.info)?;
    self.name.write(&mut w.child())?;
    self.value.write(&mut w.child())
  }
}

impl Alias<()> {
  pub fn any(args: TypeConfig) -> impl Strategy<Value = Self> {
    (Ident::any(), Type::any(args)).prop_map(|(name, value)| Alias {
      name,
      value: Box::new(value),
      info: (),
    })
  }
}

/// A type definition that defines a new tag type
///
/// ```ignore
/// type Vec2_1 {x: Float, y: Float};
/// type Vec3_1 [Float, Float, Float];
/// type VecN_1 [Float; _];
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct NewType<I> {
  pub tag: Tagged<I>,
  pub info: I,
}

impl<I> Display for NewType<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("type ")?;
    self.tag.fmt(f)
  }
}

impl DisplayAs<Spans> for NewType<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("tag def", &self.info)?;
    self.tag.write(&mut w.child())
  }
}

impl NewType<()> {
  pub fn any(args: TypeConfig) -> impl Strategy<Value = Self> {
    Tagged::any(Type::any(args)).prop_map(|tag| NewType { tag, info: () })
  }
}

/// The definition for a type alias
///
/// ```ignore
/// type Number = Float;
/// type Boolean = Bool;
///
/// type Vec2_0 = {x: Float, y: Float};
/// type Vec2_1 {x: Float, y: Float};
///
/// type Vec3_0 = [Float, Float, Float];
/// type Vec3_1 [Float, Float, Float];
///
/// type VecN_0 = [Float; _];
/// type VecN_1 [Float; _];
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub enum TypeDef<I> {
  Alias(Alias<I>),
  New(NewType<I>),
}

impl<I> Display for TypeDef<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Alias(a) => a.fmt(f),
      Self::New(n) => n.fmt(f),
    }
  }
}

impl DisplayAs<Spans> for TypeDef<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    match self {
      Self::Alias(a) => a.write(w),
      Self::New(n) => n.write(w),
    }
  }
}

impl TypeDef<()> {
  pub fn any(args: TypeConfig) -> impl Strategy<Value = Self> {
    prop_oneof![
      Alias::any(args).prop_map(Self::Alias),
      NewType::any(args).prop_map(Self::New),
    ]
  }
}
