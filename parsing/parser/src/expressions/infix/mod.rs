use crate::In;
use diom_syntax::expressions::Expression;

mod assign;
pub use assign::{parse_assign, PartialAssign};
mod method;
pub use method::PartialMethod;

pub enum PartialInfix<I> {
  Method(PartialMethod<I>),
  Assign(PartialAssign),
}

impl<'a> PartialInfix<In<'a>> {
  /// Applies this infix to an existing expression.\
  /// **Safety**: both `self` and `value` must be from the same input slice
  pub unsafe fn apply(
    self,
    value: Expression<In<'a>>,
    other: Expression<In<'a>>,
  ) -> Expression<In<'a>> {
    match self {
      Self::Method(method) => Expression::Infix(method.apply(value, other)),
      Self::Assign(assign) => Expression::Assign(assign.apply(value, other)),
    }
  }
}
