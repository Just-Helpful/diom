use super::Type;

/// A type for arrays of items.
///
/// ```ignore
/// let String: [Char];
/// let xs: [Float] = [1, 2, 3];
/// ```
pub struct Array<I> {
  pub item: Box<Type<I>>,
  pub info: I,
}
