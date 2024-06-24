use crate::{ident::Ident, InfoSource};

use super::Type;

/// A type for combinations of possible types
///
/// ```ignore
/// let CharOption {
///   Some(Char),
///   None,
/// };
///
/// let Boolean: {
///   True,
///   False,
/// };
///
/// let c_optn = CharOption.Some('v');
/// let c_optn = CharOption.None;
///
/// let bool = Boolean.True;
/// let bool = Boolean.False;
/// ```
#[derive(InfoSource)]
pub struct Enum<I> {
  pub name: Option<Ident<I>>,
  pub variants: Vec<(Ident<I>, Type<I>)>,
  pub info: I,
}
