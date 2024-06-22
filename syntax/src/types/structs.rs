use crate::ident::Ident;

use super::Type;

/// The type for a combination of named fields.
///
/// ```ignore
/// let Vec2 {
///   x: Float,
///   y: Float,
/// };
/// let Email: {
///   name: [Char],
///   domain: [Char],
/// };
///
/// let vec2: Vec2 = Vec2 { x: 1.2, y: 3.0 };
/// let bobs_email: Email = { name: "bob.jones", domain: "hotmail.com" };
/// ```
pub struct Struct<I> {
  pub name: Option<Ident<I>>,
  pub fields: Vec<(Ident<I>, Type<I>)>,
  pub info: I,
}
