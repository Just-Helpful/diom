use crate::{common::PResult, errors::SyntaxError, In};
use diom_syntax::expressions::Expression;
use nom::{branch::alt, Parser};

mod let_decl;
pub use let_decl::parse_let;
mod returns;
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
