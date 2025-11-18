use super::Type;
use crate::ident::Ident;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};

/// The definition for a type alias
///
/// ```_,ignore
/// let Number: Float;
/// let Boolean: Bool;
///
/// let Vec2_0: {x: Float, y: Float};
/// let Vec2_1 {x: Float, y: Float};
///
/// let Vec3_0: (Float, Float, Float);
/// let Vec3_1(Float, Float, Float);
///
/// let VecN_0: [Float];
/// let VecN_1[Float];
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct TypeDef<I> {
    pub name: Ident<I>,
    pub value: Box<Type<I>>,
    pub info: I,
}
