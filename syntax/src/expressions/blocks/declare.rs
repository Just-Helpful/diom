use crate::{patterns::Pattern, types::Type, InfoSource};

use super::Expression;

/// Decleration should allow for pattern matching in its syntax
///
/// ```ignore
/// let Vec2D { x: Float, y: Float };
/// let Vec2D {x, y} = vec0;
/// ```
///
/// In the case where it's possible for the pattern to not match during runtime
/// (most commonly for `enum`-like values), the `let` statement returns a
/// `Boolean` value (`True` if the `let` statement matches, `False` otherwise).
/// For example:
///
/// ```ignore
/// assert (let Some(x) = Some(5)) == True
/// assert (let Some(x) = None) == False
/// ```
///
/// If the type checker can prove that this value will always be `True`,
/// then it will allow the return value to remain unused, otherwise if the
/// return value is not used, it'll throw an compiler error.
#[derive(InfoSource)]
pub struct Declare<I> {
  pub pattern: Pattern<I>,
  pub annotation: Option<Type<I>>,
  pub value: Box<Expression<I>>,
  pub info: I,
}
