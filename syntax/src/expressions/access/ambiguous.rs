use crate::{expressions::Expression, ident::Ident};

/// A node to represent a syntax that could be either a
/// method call or field access.
///
/// More specifically, the syntax that causes issues is `{Expression} {Ident}`,<br>
/// as it could be either intepreted as:
/// - `{Expression}.{Ident}`, i.e. a field access
/// - `{Expression}({Ident})`, i.e. a function call with a variable
///
/// For example `let y = 3; x y` can be interpreted as either:
/// 1. `x.y` if `x: {y: Float}`
/// 2. `x(y)` if `x: (y: Float): ()`
///
/// This ambiguity **should** be resolved by typechecking, i.e. this node
/// shouldn't exist in the syntax tree after the typechecking stage.
pub struct Ambiguous<I> {
  pub value: Box<Expression<I>>,
  pub name: Ident<I>,
  pub info: I,
}
