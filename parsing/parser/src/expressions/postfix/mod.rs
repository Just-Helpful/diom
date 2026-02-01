use crate::In;
use diom_syntax::expressions::Expression;

pub enum PartialPostFix<I> {
  Call(call::PostFixCall<I>),
  Field(field::PostFixField<I>),
  Index(index::PostFixIndex<I>),
}

impl<'a> PartialPostFix<In<'a>> {
  /// Applies this postfix to an existing expression.\
  /// **Safety**: both `self` and `value` must be from the same input slice
  pub unsafe fn apply(self, value: Expression<In<'a>>) -> Expression<In<'a>> {
    match self {
      Self::Call(c) => Expression::Call(c.apply(value)),
      Self::Field(f) => Expression::Field(f.apply(value)),
      Self::Index(i) => Expression::Index(i.apply(value)),
    }
  }
}

mod call;
pub use call::{parse_explicit_call, parse_implicit_call};
mod field;
pub use field::parse_field;
mod index;
pub use index::parse_index;
