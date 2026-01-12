use crate::{common::PResult, errors::SyntaxError, In};
use diom_syntax::expressions::Expression;
use nom::{branch::alt, error::context, Parser};

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
  context(
    "compound value",
    alt((
      context("array", parse_array.map(Expression::Array)),
      context("struct", parse_struct.map(Expression::Struct)),
      context("function", parse_function.map(Expression::Function)),
      context("block", parse_block.map(Expression::Block)),
    )),
  )
  .parse(input)
}
