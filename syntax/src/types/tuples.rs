use crate::{ident::Ident, InfoSource};

use super::Type;

/// The type for a combination of indexed fields
///
/// ```ignore
/// let Vec2(Float, Float);
/// let Email: ([Char], [Char]);
///
/// let vec2: Vec2 = Vec2(1.2, 3.0);
/// let bobs_email: Email = ("bob.jones", "hotmail.com");
/// ```
#[derive(InfoSource, Clone)]
pub struct Tuple<I> {
  pub name: Option<Ident<I>>,
  pub fields: Vec<Type<I>>,
  pub info: I,
}
