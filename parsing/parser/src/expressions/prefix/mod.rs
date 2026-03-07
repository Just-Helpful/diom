use crate::In;
use diom_syntax::expressions::Expression;

mod declare;
pub use declare::parse_let;
use declare::PartialDeclare;
mod op;
pub use op::PartialPrefixOp;
mod returns;
pub use returns::parse_return;
use returns::PartialReturn;

/// A prefix to a given expression, either:
/// 1. `return ...`
/// 2. `let <pattern>: <type> = ...`
/// 3. maybe negations `- ...` / `! ...` ???
pub enum PartialPrefix<I> {
  Return(PartialReturn<I>),
  Declare(PartialDeclare<I>),
  Op(PartialPrefixOp<I>),
}

impl<'a> PartialPrefix<In<'a>> {
  /// Applies this prefix to an existing expression.\
  /// **Safety**: both `self` and `value` must be from the same input slice
  pub unsafe fn apply(self, value: Expression<In<'a>>) -> Expression<In<'a>> {
    match self {
      Self::Return(r) => Expression::Return(r.apply(value)),
      Self::Declare(d) => Expression::Declare(d.apply(value)),
      Self::Op(o) => Expression::Call(o.apply(value)),
    }
  }
}
