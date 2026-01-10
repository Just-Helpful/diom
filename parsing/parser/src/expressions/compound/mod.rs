use crate::{common::PResult, errors::SyntaxError, In};
use diom_syntax::expressions::Expression;
use nom::{branch::alt, Parser};

mod arrays;
pub use arrays::parse_array;
mod structs;
pub use structs::parse_struct;
pub mod function;
pub use function::parse_function;
mod block;
pub use block::parse_block;

pub fn parse_compound_value<'a, E: SyntaxError<'a>>(
  input: In<'a>,
) -> PResult<'a, Expression<In<'a>>, E> {
  alt((
    parse_array.map(Expression::Array),
    parse_struct.map(Expression::Struct),
    parse_function.map(Expression::Function),
    parse_block.map(Expression::Block),
  ))
  .parse(input)
}
