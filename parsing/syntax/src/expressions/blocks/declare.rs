use crate::{expressions::Expression, patterns::Pattern, scope::SyntaxScope, types::Type};

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
#[derive(Clone, Debug)]
pub struct Declare<S: SyntaxScope> {
  pub pattern: S::Single<Pattern<S>>,
  pub annotation: Option<S::Single<Type<S>>>,
  pub value: S::Single<Expression<S>>,
}

// impl DisplayAs<Spans> for Declare<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("declare", &self.info)?;
//     self.pattern.write(&mut w.child())?;
//     self.annotation.write(&mut w.child())?;
//     self.value.write(&mut w.child())
//   }
// }
