mod arrays;
pub use arrays::parse_array;
mod structs;
use diom_syntax::expressions::Expression;
use diom_tokens::SpanTokens;
use nom::{branch::alt, Parser};
pub use structs::parse_struct;
pub mod function;
pub use function::parse_function;
mod group;
pub use group::parse_group;

use crate::{common::PResult, Span};

pub fn parse_compound_value(input: SpanTokens) -> PResult<Expression<Span>> {
  alt((
    parse_array.map(Expression::Array),
    parse_struct.map(Expression::Struct),
    parse_function.map(Expression::Function),
    parse_group.map(Expression::Group),
  ))(input)
}
