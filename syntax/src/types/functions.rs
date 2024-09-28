use crate::ident::Ident;
use diom_info::{InfoMap, InfoRef, InfoSource};

use super::Type;

#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Argument<I> {
  pub name: Ident<I>,
  pub annotation: Type<I>,
  pub info: I,
}

/// The type for a callable function
///
/// ```ignore
/// # function types can be simplified a bit
/// let Binary: (x: Float): (y: Float): Float;
/// let Binary: (x: Float)(y: Float): Float;
/// let Binary(x: Float)(y: Float): Float;
///
/// let add: Binary = (x)(y) x + y;
/// let add: Binary = (x) {(y) {x + y}};
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Function<I> {
  pub arguments: Vec<Argument<I>>,
  pub returned: Box<Type<I>>,
  pub info: I,
}
