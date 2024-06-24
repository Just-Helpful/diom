use crate::{expressions::Expression, ident::Ident, InfoSource};

/// A node to represent a syntax that could be either a
/// method call or field access.
///
/// More specifically, the ambiguous syntax is `{Expression} {Ident}`,<br>
/// as it could be either intepreted as:
///
/// - `{Expression}.{Ident}`, i.e. a field access
/// - `{Expression}({Ident})`, i.e. a function call with a variable
///
/// For example `let y = 3; x y` can be interpreted as either:
///
/// - `x.y` if `x: {y: Float}`
/// - `x(y)` if `x: (y: Float): ()`
///
/// This ambiguity **should** be resolved by typechecking, i.e. this node<br>
/// **shouldn't** exist in the syntax tree after the typechecking stage.
#[derive(InfoSource)]
pub struct Ambiguous<I> {
  pub value: Box<Expression<I>>,
  pub name: Ident<I>,
  pub info: I,
}
