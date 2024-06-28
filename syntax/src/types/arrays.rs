use super::Type;
use crate::{ident::Ident, InfoSource};

/// A type for arrays of items.
///
/// ```ignore
/// let String: [Char];
/// let xs: [Float] = [1, 2, 3];
/// ```
#[derive(InfoSource)]
pub struct Array<I> {
  pub name: Option<Ident<I>>,
  pub item: Box<Type<I>>,
  pub info: I,
}
