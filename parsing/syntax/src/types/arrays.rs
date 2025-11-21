use super::Type;
use crate::ident::Ident;
use diom_info_traits::{InfoMap, InfoRef, InfoSource};

/// A type for arrays of items.
///
/// ```ignore
/// type String [Char; _];
/// type Nums = [Float];
///
/// let greeting: String = "Hello!";
/// let xs: Nums = [1, 2, 3];
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap, Debug)]
pub struct Array<I> {
  pub name: Option<Ident<I>>,
  pub item: Box<Type<I>>,
  pub info: I,
}
