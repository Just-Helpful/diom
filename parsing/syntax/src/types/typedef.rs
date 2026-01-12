use super::Type;
use crate::fmt::{bracket, OptionsDisplay};
use crate::ident::Ident;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};
use std::ops::Range;

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

impl OptionsDisplay for TypeDef<Range<usize>> {
  type Options = usize;
  fn optn_fmt(&self, w: &mut crate::fmt::MultiWriter, depth: Self::Options) -> std::fmt::Result {
    w.write_at(
      [self.info.start, depth],
      bracket("type def", self.info.len()),
    );
    self.name.optn_fmt(w, depth + 1)?;
    self.value.optn_fmt(w, depth + 1)
  }
}
