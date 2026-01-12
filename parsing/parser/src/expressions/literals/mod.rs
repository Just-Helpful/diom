use crate::{common::PResult, errors::SyntaxError, ident::parse_ident, In};
use diom_syntax::expressions::Expression;
use nom::{branch::alt, error::context, Parser};

mod chars;
use chars::parse_char;
mod floats;
use floats::parse_float;

pub fn parse_literal_value<'a, E: SyntaxError<'a>>(
  input: In<'a>,
) -> PResult<'a, Expression<In<'a>>, E> {
  context(
    "literal value",
    alt((
      parse_char.map(Expression::Char),
      parse_float.map(Expression::Float),
      parse_ident.map(Expression::Var),
    )),
  )
  .parse(input)
}
