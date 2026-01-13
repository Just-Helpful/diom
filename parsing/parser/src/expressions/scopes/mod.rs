use crate::{common::PResult, errors::SyntaxError, In};

mod let_decl;
use diom_syntax::expressions::Expression;
pub use let_decl::parse_let;
mod returns;
use nom::{branch::alt, Parser};
pub use returns::parse_return;

pub fn parse_scope_value<'a, E: SyntaxError<'a>>(
  input: In<'a>,
) -> PResult<'a, Expression<In<'a>>, E> {
  alt((
    parse_let.map(Expression::Declare),
    parse_return.map(Expression::Return),
  ))
  .parse(input)
}
