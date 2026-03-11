use super::Type;
use crate::ident::Ident;
use diom_fmt::{DisplayAs, SpanWriter, Spans};
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::{
  fmt::{Display, Write},
  ops::Range,
};

/// The definition for a type alias
///
/// ```_,ignore
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
pub struct TypeDef<I> {
  pub name: Ident<I>,
  pub value: Box<Type<I>>,
  pub info: I,
}

impl<I> Display for TypeDef<I> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str("type ")?;
    self.name.fmt(f)?;
    f.write_str("=")?;
    self.value.fmt(f)
  }
}

impl DisplayAs<Spans> for TypeDef<Range<usize>> {
  fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
    w.bracket("type def", &self.info)?;
    self.name.write(&mut w.child())?;
    self.value.write(&mut w.child())
  }
}
