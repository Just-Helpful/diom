use crate::ident::Ident;
use diom_info::{InfoMap, InfoRef, InfoSource};

/// A pattern that captures the remaining contents of a structure.
/// For example:
/// ```ignore
/// let [1, *xs] = [1, 2, 3];
/// assert(xs == [2, 3]);
///
/// let {x: 1, *sx} = {x: 1, y: 2, z: 3};
/// assert(sx == {y: 2, z: 3});
///
/// let (1, *tx) = (1, True, "");
/// assert(tx == (True, ""));
/// ```
///
/// @note as diom doesn't allow empty structs, attempting to bind a rest<br>
/// parameter that would create one is a compiler error:
///
/// ```ignore
/// let {x: 1, *sx} = {x: 1};
/// // gives a compiler error (whilst type checking)
/// // that `sx` cannot be bound to an empty structure
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Rest<I> {
  pub name: Option<Ident<I>>,
  pub info: I,
}
