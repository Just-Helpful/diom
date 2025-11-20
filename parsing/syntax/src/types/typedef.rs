use super::Type;
use crate::ident::Ident;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};

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
#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct TypeDef<I> {
  pub name: Ident<I>,
  pub value: Box<Type<I>>,
  pub info: I,
}
