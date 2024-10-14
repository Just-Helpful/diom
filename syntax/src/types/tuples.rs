use crate::ident::Ident;
use diom_info::{InfoMap, InfoRef, InfoSource};

use super::Type;

/// The type for a combination of indexed fields
///
/// ```ignore
/// let Vec2[Float, Float];
/// let Email: [String, String];
///
/// let vec2: Vec2 = Vec2[1.2, 3.0];
/// let bobs_email: Email = ["bob.jones", "hotmail.com"];
/// ```
#[derive(Clone, InfoSource, InfoRef, InfoMap)]
pub struct Tuple<I> {
  pub name: Option<Ident<I>>,
  pub fields: Vec<Type<I>>,
  pub info: I,
}
